//! Integration tests for SigmaOS-emacs-ui-theme-directory.

use sigmaos_emacs_ui_theme_directory_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
