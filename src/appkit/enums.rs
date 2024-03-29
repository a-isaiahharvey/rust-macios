use objc::Encode;

use crate::foundation::UInt;

/// Constants that indicate whether a copy or print operation was successful,
/// was canceled, or failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationDelegateReply {
    /// Indicates the operation succeeded.
    Success = 0,
    /// Indicates the user cancelled the operation.
    Cancel = 1,
    /// Indicates an error occurred processing the operation.
    Failure = 2,
}

/// Constants for the types of events that responder objects can handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSEventType {
    /// The user pressed the left mouse button.
    LeftMouseDown = 1,
    /// The user released the left mouse button.
    LeftMouseUp = 2,
    /// The user pressed the right mouse button.
    RightMouseDown = 3,
    /// The user released the right mouse button.
    RightMouseUp = 4,
    /// The user moved the mouse in a way that caused the cursor to move onscreen.
    MouseMoved = 5,
    /// The user moved the mouse while holding down the left mouse button.
    LeftMouseDragged = 6,
    /// The user moved the mouse while holding down the right mouse button.
    RightMouseDragged = 7,
    /// The cursor entered a well-defined area, such as a view.
    MouseEntered = 8,
    /// The cursor exited a well-defined area, such as a view.
    MouseExited = 9,
    /// The user pressed a key on the keyboard.
    KeyDown = 10,
    /// The user released a key on the keyboard.
    KeyUp = 11,
    /// The event flags changed.
    FlagsChanged = 12,
    /// An AppKit-related event occurred.
    AppKitDefined = 13,
    /// A system-related event occurred.
    SystemDefined = 14,
    /// An app-defined event occurred.
    ApplicationDefined = 15,
    /// An event that provides execution time to periodic tasks.
    Periodic = 16,
    /// An event that updates the cursor.
    CursorUpdate = 17,

    /// The scroll wheel position changed.
    ScrollWheel = 22,

    /// The user touched a point on a tablet.
    TabletPoint = 23,
    /// A pointing device is near, but not touching, the associated tablet.
    TabletProximity = 24,

    /// The user pressed a tertiary mouse button.
    OtherMouseDown = 25,
    /// The user released a tertiary mouse button.
    OtherMouseUp = 26,
    /// The user moved the mouse while holding down a tertiary mouse button.
    OtherMouseDragged = 27,

    /// The user performed a nonspecific type of gesture.
    Gesture = 29,
    /// The user performed a pinch-open or pinch-close gesture.
    Magnify = 30,
    /// The user performed a swipe gesture.
    Swipe = 31,
    /// The user performed a rotate gesture.
    Rotate = 18,
    /// An event marking the beginning of a gesture.
    BeginGesture = 19,
    /// An event that marks the end of a gesture.
    EndGesture = 20,

    /// The user performed a smart-zoom gesture.
    SmartMagnify = 32,
    /// An event that initiates a Quick Look request.
    QuickLook = 33,
    /// An event that reports a change in pressure on a pressure-sensitive device.
    Pressure = 34, // 10.10.3, 64-bit-only
    /// The user touched a portion of the touch bar.
    DirectTouch = 37, // 10.10
    /// The user changed the mode of a connected device.
    ChangeMode = 38,
}

/// Constants that you use to filter out specific event types from the stream
/// of incoming events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSEventMask {
    /// A mask for left mouse-down events.
    LeftMouseDown = 1 << NSEventType::LeftMouseDown as u64,
    /// A mask for left mouse-up events.
    LeftMouseUp = 1 << NSEventType::LeftMouseUp as u64,
    /// A mask for right mouse-down events.
    RightMouseDown = 1 << NSEventType::RightMouseDown as u64,
    /// A mask for right mouse-up events.
    RightMouseUp = 1 << NSEventType::RightMouseUp as u64,
    /// A mask for mouse-moved events.
    MouseMoved = 1 << NSEventType::MouseMoved as u64,
    /// A mask for left mouse-dragged events.
    LeftMouseDragged = 1 << NSEventType::LeftMouseDragged as u64,
    /// A mask for right mouse-dragged events.
    RightMouseDragged = 1 << NSEventType::RightMouseDragged as u64,
    /// A mask for mouse-entered events.
    MouseEntered = 1 << NSEventType::MouseEntered as u64,
    /// A mask for mouse-exited events.
    MouseExited = 1 << NSEventType::MouseExited as u64,
    /// A mask for key-down events.
    KeyDown = 1 << NSEventType::KeyDown as u64,
    /// A mask for key-up events.
    KeyUp = 1 << NSEventType::KeyUp as u64,
    /// A mask for flags-changed events.
    FlagsChanged = 1 << NSEventType::FlagsChanged as u64,
    /// A mask for AppKit–defined events.
    AppKitDefined = 1 << NSEventType::AppKitDefined as u64,
    /// A mask for system-defined events.
    SystemDefined = 1 << NSEventType::SystemDefined as u64,
    /// A mask for app-defined events.
    ApplicationDefined = 1 << NSEventType::ApplicationDefined as u64,
    /// A mask for periodic events.
    Periodic = 1 << NSEventType::Periodic as u64,
    /// A mask for cursor-update events.
    CursorUpdate = 1 << NSEventType::CursorUpdate as u64,
    /// A mask for scroll-wheel events.
    ScrollWheel = 1 << NSEventType::ScrollWheel as u64,
    /// A mask for tablet-point events.
    TabletPoint = 1 << NSEventType::TabletPoint as u64,
    /// A mask for tablet-proximity events.
    TabletProximity = 1 << NSEventType::TabletProximity as u64,
    /// A mask for tertiary mouse-down events.
    OtherMouseDown = 1 << NSEventType::OtherMouseDown as u64,
    /// A mask for right mouse-up events.
    OtherMouseUp = 1 << NSEventType::OtherMouseUp as u64,
    /// A mask for tertiary mouse-dragged events.
    OtherMouseDragged = 1 << NSEventType::OtherMouseDragged as u64,
    /// A mask for generic gesture events.
    Gesture = 1 << NSEventType::Gesture as u64,
    /// A mask for magnify-gesture events.
    Magnify = 1 << NSEventType::Magnify as u64,
    /// A mask for swipe-gesture events.
    Swipe = 1 << NSEventType::Swipe as u64,
    /// A mask for rotate-gesture events.
    Rotate = 1 << NSEventType::Rotate as u64,
    /// A mask for begin-gesture events.
    BeginGesture = 1 << NSEventType::BeginGesture as u64,
    /// A mask for end-gesture events.
    EndGesture = 1 << NSEventType::EndGesture as u64,
    /// A mask for smart-zoom gesture events.
    SmartMagnify = 1 << NSEventType::SmartMagnify as u64,
    /// A mask for pressure-change events.
    Pressure = 1 << NSEventType::Pressure as u64, // 10.10.3, 64-bit-only
    /// A mask for touch events.
    DirectTouch = 1 << NSEventType::DirectTouch as u64, // 10.10
    /// A mask for change-mode events.
    ChangeMode = 1 << NSEventType::ChangeMode as u64,
    /// A mask that matches any type of event.
    AnyEvent = UInt::max_value(),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
/// Activation policies (used by `activationPolicy`) that control whether and how an app may be activated.
pub enum NSApplicationActivationPolicy {
    /// The application is an ordinary app that appears in the Dock and may have a user interface.
    Regular,
    /// The application doesn’t appear in the Dock and doesn’t have a menu bar, but it may be activated programmatically or by clicking on one of its windows.
    Accessory,
    /// The application doesn’t appear in the Dock and may not create windows or be activated.
    Prohibited,
}

/// The following flags are for `activateWithOptions`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationActivationOptions {
    /// By default, activation brings only the main and key windows forward.
    /// If you specify NSApplicationActivateAllWindows, all of the
    /// application's windows are brought forward.
    AllWindows = 1 << 0,
    /// The application is activated regardless of the currently active app.
    IgnoringOtherWindows = 1 << 1,
}

/// Constants that determine whether an app should terminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationTerminateReply {
    /// The app should not be terminated.
    Cancel,
    /// It is OK to proceed with termination.
    Now,
    /// The app should be terminated, but the user should be asked first.
    Later,
}

unsafe impl Encode for NSApplicationTerminateReply {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("q") }
    }
}

/// Constants that specify the style of a window, and that you can combine
/// with the C bitwise OR operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowStyleMask {
    /// The window displays none of the usual peripheral elements. Useful only
    /// for display or caching purposes. A window that uses
    /// NSWindowStyleMaskBorderless can’t become key or main, unless the
    /// value of canBecomeKeyWindow or canBecomeMainWindow is YES. Note that
    /// you can set a window’s or panel’s style mask to
    /// NSWindowStyleMaskBorderless in Interface Builder by deselecting Title
    /// Bar in the Appearance section of the Attributes inspector.
    Borderless = 0,
    /// The window displays a title bar.
    Titled = 1 << 0,
    /// The window displays a close button.
    Closable = 1 << 1,
    /// The window displays a minimize button.
    Miniaturizable = 1 << 2,
    /// The window can be resized by the user.
    Resizable = 1 << 3,
    /// The window is a panel or a subclass of NSPanel.
    Utility = 1 << 4,
    /// The window is a document-modal panel (or a subclass of NSPanel).
    DocModal = 1 << 6,
    /// The window is a panel or a subclass of NSPanel that does not activate
    /// the owning app.
    NonactivatingPanel = 1 << 7,
    /// The window uses a textured background that darkens when the window is
    /// key or main and lightens when it is inactive, and may have a second
    /// gradient in the section below the window content.
    #[deprecated]
    TexturedBackground = 1 << 8,
    ///
    Unscaled = 1 << 11,
    /// This constant has no effect, because all windows that include a
    /// toolbar use the unified style.
    UnifiedTitleAndToolbar = 1 << 12,
    /// The window is a HUD panel.
    Hud = 1 << 13,
    /// The window can appear full screen. A fullscreen window does not
    /// draw its title bar, and may have special handling for its toolbar.
    /// (This mask is automatically toggled when toggleFullScreen: is called.)
    FullScreenWindow = 1 << 14,
    /// When set, the window’s contentView consumes the full size of the
    /// window. Although you can combine this constant with other window
    /// style masks, it is respected only for windows with a title bar.
    /// Note that using this mask opts in to layer-backing. Use the
    /// contentLayoutRect or the contentLayoutGuide to lay out views
    /// underneath the title bar–toolbar area.
    FullSizeContentView = 1 << 15,
}

/// Styles that determine the appearance and location of the toolbar in
/// relation to the title bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSWindowToolbarStyle {
    /// A style indicating that the system determines the toolbar’s appearance
    /// and location.
    Automatic,
    /// A style indicating that the toolbar appears below the window title.
    Expanded,
    /// A style indicating that the toolbar appears below the window title with
    /// toolbar items centered in the toolbar.
    Preference,
    /// A style indicating that the toolbar appears next to the window title.
    Unified,
    /// A style indicating that the toolbar appears next to the window title
    /// and with reduced margins to allow more focus on the window’s contents.
    UnifiedCompact,
}

/// Specifies the appearance of the window’s title bar area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSWindowTitleVisibility {
    /// The window has the regular window title and title bar buttons.
    Visible = 0,
    /// The window has no title bar buttons.
    Hidden = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
/// Constants that specify how the window device buffers the drawing done in a window.
pub enum NSBackingStoreType {
    /// The window uses a buffer, but draws directly to the screen where possible and to the buffer for obscured portions.
    #[deprecated]
    Retained = 0,
    /// The window draws directly to the screen without using any buffer.
    #[deprecated]
    Nonretained = 1,
    /// The window renders all drawing into a display buffer and then flushes it to the screen.
    Buffered = 2,
}

/// Window collection behaviors related to Exposé and Spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowCollectionBehavior {
    /// The window appears in only one space at a time.
    Default = 0,
    /// The window appears in all spaces.
    CanJoinAllSpaces = 1 << 0,
    /// When the window becomes active, move it to the active space instead of switching spaces.
    MoveToActiveSpace = 1 << 1,
    /// The window participates in Spaces and Exposé.
    Managed = 1 << 2,
    /// The window floats in Spaces and hides in Exposé.
    Transient = 1 << 3,
    /// Exposé doesn’t affect the window, so it stays visible and stationary, like the desktop window.
    Stationary = 1 << 4,
    /// The window participates in the window cycle for use with the Cycle Through Windows menu item.
    ParticipatesInCycle = 1 << 5,
    /// The window isn’t part of the window cycle for use with the Cycle Through Windows menu item.
    IgnoresCycle = 1 << 6,
    /// The window can enter full-screen mode.
    FullScreenPrimary = 1 << 7,
    /// The window can display on the same space as the full-screen window.
    FullScreenAuxiliary = 1 << 8,
    /// The window doesn’t support full-screen mode.
    FullScreenNone = 1 << 9,
    /// The window can be a secondary full screen tile even if it can’t be a full screen window itself.
    FullScreenAllowsTiling = 1 << 11,
    /// The window doesn’t support being a full-screen tile window, but may support being a full-screen window.
    FullScreenDisallowsTiling = 1 << 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// A type that represents the depth, or amount of memory, for a single pixel in a window or screen.
pub enum NSWindowDepth {
    /// Twenty four bit RGB depth limit.
    TwentyfourBitRgb = 0x208,
    /// Sixty four bit RGB depth limit.
    SixtyfourBitRgb = 0x210,
    /// One hundred and twenty eight bit RGB depth limit.
    OneHundredTwentyEightBitRgb = 0x220,
}

/// Options to use when retrieving window numbers from the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowNumberListOptions {
    ///
    AllApplication = 1 << 0,
    ///
    AllSpaces = 1 << 4,
}

/// Constants that represent the access levels other processes can have to a window’s content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowSharingType {
    /// The window’s contents cannot be read by another process.
    None,
    ///
    ReadOnly,
    ///
    ReadWrite,
}

/// Constants that let you specify how a window is ordered relative to another window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSWindowOrderingMode {
    /// Moves the window below the indicated window.
    Below = -1,
    /// Moves the window off the screen.
    Out,
    /// Moves the window above the indicated window.
    Above,
}

/// Specifies whether the window is occluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowOcclusionState {
    /// If set, at least part of the window is visible; if not set, the entire window is occluded. A window that has a nonrectangular shape can be entirely occluded onscreen, but if its bounding box falls into a visible region, the window is considered to be visible. Note that a completely transparent window may also be considered visible.
    Visible = 1 << 1,
}

/// Constants that provide a way to access standard title bar buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSWindowButton {
    /// The close button.
    CloseButton,
    /// The minimize button.
    MiniaturizeButton,
    /// The zoom button.
    ZoomButton,
    /// The toolbar button.
    ToolbarButton,
    /// The document icon button.
    DocumentIconButton,
    /// The document versions button.
    DocumentVersionsButton = 6,
    /// The fullscreen icon button.
    #[deprecated = "The standard window button for FullScreenButton is always null; use ZoomButton instead."]
    FullScreenButton,
}

/// Styles that determine the type of separator displayed between the title bar and content of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSTitlebarSeparatorStyle {
    /// A style indicating that the system determines the type of separator.
    Automatic,
    /// A style indicating that there’s no title bar separator.
    None,
    /// A style indicating that there’s no title bar separator.
    Line,
    /// A style indicating that the title bar separator is a shadow.
    Shadow,
}

/// Specifies the directional flow of the user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSUserInterfaceLayoutDirection {
    /// Layout direction is left to right.
    LeftToRight,
    /// Layout direction is right to left.
    RightToLeft,
}

/// The appearance and disappearance behavior of a popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum NSPopoverBehavior {
    /// Your application assumes responsibility for closing the popover.
    ApplicationDefined = 0,
    /// The system will close the popover when the user interacts with a user interface element outside the popover.
    Transient = 1,
    /// The system will close the popover when the user interacts with user interface elements in the window containing the popover's positioning view.
    Semitransient = 2,
}

/// A set of optional status item behaviors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NSStatusItemBehavior {
    /// A status item that allows interactive removal.
    RemovalAllowed = (1 << 1),
    /// A status item that quits the application upon removal.
    TerminationOnRemoval = (1 << 2),
}
