use iced::Font;

pub const THIN: Font = Font::External {
    name: "IBM Plex Sans KR Thin",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Thin.ttf"
    )),
};

pub const EXTRA_LIGHT: Font = Font::External {
    name: "IBM Plex Sans KR ExtraLight",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-ExtraLight.ttf"
    )),
};

pub const LIGHT: Font = Font::External {
    name: "IBM Plex Sans KR Light",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Light.ttf"
    )),
};

pub const REGULAR: Font = Font::External {
    name: "IBM Plex Sans KR Regular",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Regular.ttf"
    )),
};

pub const TEXT: Font = Font::External {
    name: "IBM Plex Sans KR Text",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Text.ttf"
    )),
};

pub const MEDIUM: Font = Font::External {
    name: "IBM Plex Sans KR Medium",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Medium.ttf"
    )),
};

pub const SEMI_BOLD: Font = Font::External {
    name: "IBM Plex Sans KR SemiBold",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-SemiBold.ttf"
    )),
};

pub const BOLD: Font = Font::External {
    name: "IBM Plex Sans KR Bold",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/res/fonts/IBMPlexSansKR-Bold.ttf"
    )),
};
