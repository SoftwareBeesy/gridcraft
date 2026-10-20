//! The title bar's AutoSave switch follows the interface language.

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::Session;
use gridcraft_ui_egui::{SheetApp, i18n::Language};

fn harness(language: Language) -> egui_kittest::Harness<'static, SheetApp> {
    let mut session = Session::new();
    session.new_workbook();
    let mut app = SheetApp::new(session, Default::default());
    app.ui.language = language;
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

#[test]
fn the_autosave_label_is_translated() {
    let h = harness(Language::PtBr);
    assert!(h.query_by_label("Salvamento Automático").is_some());
    assert!(h.query_by_label("AutoSave").is_none());
    let h = harness(Language::En);
    assert!(h.query_by_label("AutoSave").is_some());
}

#[test]
fn the_autosave_tooltip_has_a_portuguese_translation() {
    let tip = "AutoSave: save after every change (workbooks saved to a file)";
    assert_ne!(Language::PtBr.tr(tip), tip);
    assert_eq!(Language::En.tr(tip), tip);
}
