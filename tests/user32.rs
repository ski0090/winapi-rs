// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
extern crate user32;
extern crate test;
use user32::*;
use test::black_box as bb;
#[cfg(target_arch = "x86_64")]
#[test]
fn functions_x64() {
    bb(GetClassLongPtrA);
    bb(GetClassLongPtrW);
    bb(GetWindowLongPtrA);
    bb(GetWindowLongPtrW);
    bb(SetClassLongPtrA);
    bb(SetClassLongPtrW);
    bb(SetWindowLongPtrA);
    bb(SetWindowLongPtrW);
}
#[test]
fn functions() {
    bb(ActivateKeyboardLayout);
    // bb(AddClipboardFormatListener);
    bb(AdjustWindowRect);
    bb(AdjustWindowRectEx);
    bb(AllowSetForegroundWindow);
    bb(AnimateWindow);
    bb(AnyPopup);
    bb(ArrangeIconicWindows);
    bb(AttachThreadInput);
    bb(BeginPaint);
    bb(BlockInput);
    bb(BringWindowToTop);
    // bb(CalculatePopupWindowPosition);
    bb(CascadeWindows);
    bb(ChangeClipboardChain);
    bb(ChangeDisplaySettingsExW);
    bb(ChangeDisplaySettingsW);
    bb(ChildWindowFromPoint);
    bb(ChildWindowFromPointEx);
    bb(ClipCursor);
    bb(CloseClipboard);
    bb(CloseDesktop);
    bb(CloseWindow);
    bb(CloseWindowStation);
    bb(CountClipboardFormats);
    bb(CreateCaret);
    bb(CreateCursor);
    bb(CreateWindowExW);
    bb(DefWindowProcW);
    bb(DeferWindowPos);
    bb(DeleteMenu);
    bb(DeregisterShellHookWindow);
    bb(DestroyAcceleratorTable);
    bb(DestroyCaret);
    bb(DestroyCursor);
    bb(DestroyIcon);
    bb(DestroyMenu);
    bb(DestroyWindow);
    bb(DispatchMessageW);
    bb(EmptyClipboard);
    bb(EnableScrollBar);
    bb(EnableWindow);
    bb(EndPaint);
    bb(EndTask);
    bb(EnumClipboardFormats);
    bb(EnumDisplayDevicesW);
    bb(EnumDisplaySettingsExW);
    bb(EnumThreadWindows);
    bb(EnumWindows);
    bb(FillRect);
    bb(FindWindowA );
    bb(FindWindowExA);
    bb(FindWindowExW);
    bb(FindWindowW);
    bb(GetActiveWindow);
    bb(GetAncestor);
    bb(GetAsyncKeyState);
    bb(GetCaretBlinkTime);
    bb(GetCaretPos);
    bb(GetClassInfoExW);
    bb(GetClassInfoW);
    bb(GetClassLongA);
    bb(GetClassLongW);
    bb(GetClassWord);
    bb(GetClientRect);
    bb(GetClipCursor);
    bb(GetClipboardData);
    bb(GetClipboardFormatNameA);
    bb(GetClipboardFormatNameW);
    bb(GetClipboardOwner);
    bb(GetClipboardSequenceNumber);
    bb(GetClipboardViewer);
    bb(GetCursor);
    bb(GetCursorPos);
    bb(GetDC);
    bb(GetDesktopWindow);
    bb(GetFocus);
    bb(GetForegroundWindow);
    bb(GetKBCodePage);
    bb(GetKeyNameTextA);
    bb(GetKeyNameTextW);
    bb(GetKeyState);
    bb(GetKeyboardLayout);
    bb(GetKeyboardLayoutList);
    bb(GetKeyboardLayoutNameA);
    bb(GetKeyboardLayoutNameW);
    bb(GetKeyboardState);
    bb(GetKeyboardType);
    bb(GetMessageW);
    bb(GetOpenClipboardWindow);
    bb(GetParent);
    // bb(GetPhysicalCursorPos);
    bb(GetScrollPos);
    bb(GetScrollRange);
    bb(GetShellWindow);
    bb(GetSysColor);
    bb(GetSystemMetrics);
    bb(GetThreadDesktop);
    // bb(GetUpdatedClipboardFormats);
    bb(GetWindow);
    bb(GetWindowLongA);
    bb(GetWindowLongW);
    bb(GetWindowPlacement);
    bb(GetWindowRect);
    bb(GetWindowTextA);
    bb(GetWindowTextLengthA);
    bb(GetWindowTextLengthW);
    bb(GetWindowTextW);
    bb(HideCaret);
    bb(InvalidateRect);
    bb(IsClipboardFormatAvailable);
    bb(IsIconic);
    bb(IsWindow);
    bb(IsWindowEnabled);
    bb(IsWindowVisible);
    bb(LoadCursorFromFileW);
    bb(LoadCursorW);
    bb(LoadImageA);
    bb(LoadImageW);
    bb(MessageBoxA);
    bb(MessageBoxExA);
    bb(MessageBoxExW);
    bb(MessageBoxW);
    bb(OpenClipboard);
    bb(PeekMessageW);
    bb(PostMessageW);
    bb(PostQuitMessage);
    bb(RegisterClassExW);
    bb(RegisterClipboardFormatA);
    bb(RegisterClipboardFormatW);
    bb(ReleaseDC);
    bb(ScrollDC);
    bb(ScrollWindow);
    bb(ScrollWindowEx);
    bb(SendInput);
    bb(SendMessageA);
    bb(SendMessageTimeoutA);
    bb(SendMessageTimeoutW);
    bb(SendMessageW);
    bb(SendNotifyMessageA);
    bb(SendNotifyMessageW);
    bb(SetActiveWindow);
    bb(SetCaretBlinkTime);
    bb(SetCaretPos);
    bb(SetClassLongA);
    bb(SetClassLongW);
    bb(SetClassWord);
    bb(SetClipboardViewer);
    bb(SetCursor);
    bb(SetCursorPos);
    bb(SetFocus);
    bb(SetForegroundWindow);
    // bb(SetPhysicalCursorPos);
    bb(SetScrollPos);
    bb(SetScrollRange);
    bb(SetSystemCursor);
    bb(SetWindowLongA);
    bb(SetWindowLongW);
    bb(SetWindowPos);
    bb(SetWindowTextW);
    bb(ShowCaret);
    bb(ShowCursor);
    bb(ShowWindow);
    bb(ShowWindowAsync);
    bb(TranslateMessage);
    bb(UnregisterClassA);
    bb(UnregisterClassW);
    bb(UpdateWindow);
    bb(WaitMessage);
}