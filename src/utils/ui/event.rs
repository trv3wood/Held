use std::{io, sync::MutexGuard};

use crate::utils::{buffer::LineState, cursor::CursorCrtl, style::StyleManager};

use super::{
    mode::mode::ModeType,
    uicore::{UiCore, APP_INFO, CONTENT_WINSIZE, DEF_STYLE, UI_CMD_HEIGHT},
};

pub const TAB_STR: &'static str = "        ";

pub trait KeyEventCallback {
    fn enter(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType>{
        // 默认实现
        Ok(WarpUiCallBackType::None)
    };
    fn tab(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType>{
        // 默认实现
        Ok(WarpUiCallBackType::None)
    };
    fn backspace(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType> {
        // 默认实现
        Ok(WarpUiCallBackType::None)
    }
    fn up(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType> {
        // 默认实现
        Ok(WarpUiCallBackType::None)
    }
    fn down(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType> {
        // 默认实现
        Ok(WarpUiCallBackType::None)
    }
    fn left(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType> {
        // 默认实现
        Ok(WarpUiCallBackType::None)
    }
    fn right(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType> {
        // 默认实现
        Ok(WarpUiCallBackType::None)
    }
    fn esc(&self, ui: &mut MutexGuard<UiCore>) -> io::Result<WarpUiCallBackType>{
        // 默认实现
        Ok(WarpUiCallBackType::None)
    };
    fn input_data(
        &self,
        ui: &mut MutexGuard<UiCore>,
        data: &[u8],
    ) -> io::Result<WarpUiCallBackType>;
}

#[derive(Debug, PartialEq)]
pub enum WarpUiCallBackType {
    ChangMode(ModeType),
    Exit(bool),
    None,
}
