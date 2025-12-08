use std::rc::Rc;

use color_eyre::eyre::Result;
use slint::{Model, VecModel};

slint::include_modules!();

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let ui = MainView::new()?;
    let mut packs: Vec<PackListItemData> = ui.get_modpacks().iter().collect();
    packs.push((|| {
        let mut itm = PackListItemData::default();
        itm.name = "test item".into();
        itm
    })());
    ui.set_modpacks(Rc::new(VecModel::from(packs)).into());
    ui.run()?;
    Ok(())
}
