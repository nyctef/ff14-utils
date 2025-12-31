use ff14_utils::scrip_compare::print_script_sink_compare;

#[tokio::main]
async fn main() {
    let items = [
        (40, "Occult Potion"),
        (40, "Occult Coffer"),
        (100, "Savage Aim Materia XI"),
        (200, "Savage Aim Materia XII"),
        (100, "Savage Might Materia XI"),
        (200, "Savage Might Materia XII"),
        (100, "Heaven's Eye Materia XI"),
        (200, "Heaven's Eye Materia XII"),
        (100, "Quickarm Materia XI"),
        (200, "Quickarm Materia XII"),
        (100, "Quicktongue Materia XI"),
        (200, "Quicktongue Materia XII"),
        (1000, "Lex Temple Chain"),
        (1000, "Lex Chiton"),
        (1000, "Lex Fingerless Gloves"),
        (1000, "Lex Hose"),
        (1000, "Lex Longboots"),
        (600, "Skallic Uolosapa"),
        (1000, "La Noscean Shorthair"),
        (1000, "Town Theme (Dawntrail) Orchestrion Roll"),
    ];

    print_script_sink_compare(&items, 2000).await;
}
