mod enums;
mod output;
mod prelude;
pub mod state;
mod traits;
mod utils;

use crate::vault::prelude::*;

pub fn run(args: Args) -> Result<Message, Error> {
    let mut config = Config::load();
    let mut vaults = Vaults::load();

    match &args.command {
        Command::Vault {
            show_loc,
            name,
            location,
        } => {
            if let (Some(name), Some(location)) = (name, location) {
                vaults.create_vault(name, location)?;
                Ok(Message::ItemCreated(Item::Vault, name.to_owned()))
            } else {
                vaults.list_vaults(show_loc);
                Ok(Message::Empty)
            }
        }
        Command::Enter { name } => {
            vaults.enter_vault(name)?;
            Ok(Message::VaultEntered(name.to_owned()))
        }
        Command::Note { name } => {
            vaults
                .ref_current()?
                .create_vault_item(VaultItem::Note, name)?;
            Ok(Message::ItemCreated(Item::Note, name.to_owned()))
        }
        Command::Open { name } => {
            vaults
                .ref_current()?
                .open_note(name, config.get_editor_data())?;
            Ok(Message::Empty)
        }
        Command::Folder { name } => {
            vaults
                .ref_current()?
                .create_vault_item(VaultItem::Folder, name)?;
            Ok(Message::ItemCreated(Item::Folder, name.to_owned()))
        }
        Command::Opdir => {
            vaults.ref_current()?.open_folder()?;
            Ok(Message::Empty)
        }
        Command::Chdir { path } => {
            vaults.mut_current()?.change_folder(path)?;
            Ok(Message::FolderChanged)
        }
        Command::Remove { item_type, name } => {
            match item_type {
                Item::Vault => vaults.remove_vault(name)?,
                _ => vaults
                    .ref_current()?
                    .remove_vault_item(item_type.clone().into(), name)?,
            };
            Ok(Message::ItemRemoved(item_type.to_owned(), name.to_owned()))
        }
        Command::Rename {
            item_type,
            name,
            new_name,
        } => {
            match item_type {
                Item::Vault => vaults.rename_vault(name, new_name)?,
                _ => vaults.ref_current()?.rename_vault_item(
                    item_type.clone().into(),
                    name,
                    new_name,
                )?,
            };
            Ok(Message::ItemRenamed(
                item_type.to_owned(),
                name.to_owned(),
                new_name.to_owned(),
            ))
        }
        Command::Move {
            item_type,
            name,
            new_location,
        } => {
            match item_type {
                Item::Vault => vaults.move_vault(name, new_location)?,
                _ => vaults.ref_current()?.move_vault_item(
                    item_type.clone().into(),
                    name,
                    new_location,
                )?,
            };
            Ok(Message::ItemMoved(item_type.to_owned(), name.to_owned()))
        }
        Command::Vmove {
            item_type,
            name,
            vault_name,
        } => {
            vaults.move_to_vault(item_type, name, vault_name)?;
            Ok(Message::ItemVMoved(
                item_type.to_owned(),
                name.to_owned(),
                vault_name.to_owned(),
            ))
        }
        Command::List { item_type } => {
            vaults.ref_current()?.list(item_type);
            Ok(Message::Empty)
        }
        Command::Config { config_type, value } => {
            if config_type.is_none() {
                config.open_config()?;
                return Ok(Message::Empty);
            }

            let config_type = config_type.as_ref().unwrap();

            if let Some(value) = value {
                config.set_config(config_type, value);
                Ok(Message::ConfigSet(config_type.to_owned(), value.to_owned()))
            } else {
                let value = config.get_config(config_type);
                Ok(Message::Config(config_type.to_owned(), value))
            }
        }
        _ => Ok(Message::Empty),
    }
}
