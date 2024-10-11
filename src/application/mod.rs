use crate::{
    errors::*,
    modules::input::{InputLoader, InputMapper},
    utils::ui::AppInternalInfomation,
};
use app_dirs2::AppInfo;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, ModifierKeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use mode::{error::ErrorRenderer, ModeData, ModeKey, ModeRenderer, ModeRouter};
use smallvec::SmallVec;

use std::{
    cell::RefCell,
    collections::HashMap,
    io, mem,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use crate::{
    config::appconfig::AppSetting,
    modules::perferences::{Perferences, PerferencesManager},
    plugin::system::PluginSystem,
    utils::{file::FileManager, ui::uicore::Ui},
    view::monitor::Monitor,
    workspace::Workspace,
};

mod handler;
pub mod mode;

pub struct Application {
    file_manager: FileManager,
    bak: bool,
    ui: Arc<Ui>,
    pub workspace: Workspace,
    pub monitor: Monitor,
    pub perferences: Rc<RefCell<dyn Perferences>>,
    pub mode: ModeData,
    mode_key: ModeKey,
    mode_history: HashMap<ModeKey, ModeData>,
    input_map: HashMap<
        String,
        HashMap<
            String,
            SmallVec<[fn(&mut crate::Application) -> std::result::Result<(), Error>; 4]>,
        >,
    >,
}

impl Application {
    pub fn new(file_path: Option<String>, setting: AppSetting, args: &[String]) -> Result<Self> {
        let bak;
        let mut file = if file_path.is_some() {
            bak = true;
            FileManager::new(file_path.unwrap())?
        } else {
            bak = false;
            FileManager::new("held.tmp".to_string())?
        };

        // 将文件数据读入buf
        let buf = file.init(bak)?;

        let perferences = PerferencesManager::load()?;
        let input_map = InputLoader::load(perferences.borrow().input_config_path()?)?;
        let mut monitor = Monitor::new(perferences.clone())?;
        let workspace = Workspace::create_workspace(&mut monitor, perferences.borrow(), args)?;
        Ok(Self {
            file_manager: file,
            bak,
            ui: Ui::new(Arc::new(buf), setting),
            workspace,
            monitor,
            perferences,
            mode: ModeData::Normal,
            mode_key: ModeKey::Normal,
            mode_history: HashMap::new(),
            input_map,
        })
    }

    fn init(&mut self) -> io::Result<()> {
        // Ui::init_ui()?;
        // PluginSystem::init_system();
        // self.monitor.terminal.clear().unwrap();
        self.init_modes();
        // if !self.bak {
        //     self.ui.start_page_ui()?;
        // }

        Ok(())
    }

    fn init_modes(&mut self) {
        self.mode_history.insert(ModeKey::Normal, ModeData::Normal);
        self.mode_history
            .insert(ModeKey::Error, ModeData::Error(Error::default()));
        self.mode_history.insert(ModeKey::Exit, ModeData::Exit);
    }

    pub fn run(&mut self) -> Result<()> {
        self.init()?;

        loop {
            self.render()?;
            self.listen_event()?;

            if let ModeKey::Exit = &self.mode_key {
                return Ok(());
            }
        }

        // 主线程
        match self.ui.ui_loop() {
            Ok(store) => {
                if store {
                    let buffer = &self.ui.core.lock().unwrap().buffer;
                    self.file_manager.store(buffer)?
                } else if self.file_manager.is_first_open() {
                    self.file_manager.delete_files()?;
                }
            }
            Err(_) => {
                // 补救措施：恢复备份文件
                todo!()
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn listen_event(&mut self) -> Result<()> {
        let event = self.monitor.terminal.listen()?;
        self.handle_input(event)?;
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        if let Err(err) = ModeRouter::render(&mut self.workspace, &mut self.monitor, &mut self.mode)
        {
            ErrorRenderer::render(
                &mut self.workspace,
                &mut self.monitor,
                &mut ModeData::Error(err),
            )?;
        }
        Ok(())
    }

    pub fn switch_mode(&mut self, mode_key: ModeKey) {
        if self.mode_key == mode_key {
            return;
        }

        let mut mode = self.mode_history.remove(&mode_key).unwrap();

        mem::swap(&mut self.mode, &mut mode);

        self.mode_history.insert(self.mode_key, mode);

        self.mode_key = mode_key;
    }

    fn handle_input(&mut self, event: Event) -> Result<()> {
        let key = InputMapper::event_map_str(event);
        if key.is_none() {
            return Ok(());
        }

        let key = key.unwrap();
        if let Some(mode_key) = self.mode_key.to_string() {
            if let Some(mapper) = self.input_map.get(&mode_key) {
                if let Some(commands) = mapper.get(&key).cloned() {
                    for command in commands {
                        command(self)?;
                    }
                }
            }
        }

        Ok(())
    }
}