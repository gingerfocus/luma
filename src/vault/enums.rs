use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Item {
    Vault,
    Note,
    Folder,
}
impl Item {
    pub fn fs_name(&self) -> String {
        Into::<Item>::into(self.clone()).to_string()
    }
}

impl Into<VaultItem> for Item {
    fn into(self) -> VaultItem {
        match self {
            Item::Vault => VaultItem::Folder,
            Item::Note => VaultItem::Note,
            Item::Folder => VaultItem::Folder,
        }
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Vault => "vault".to_string(),
            Item::Note => "note".to_string(),
            Item::Folder => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VaultItem {
    Note,
    Folder,
}

impl Into<Item> for VaultItem {
    fn into(self) -> Item {
        match self {
            VaultItem::Note => Item::Note,
            VaultItem::Folder => Item::Folder,
        }
    }
}

impl ToString for VaultItem {
    fn to_string(&self) -> String {
        match self {
            VaultItem::Note => "note".to_string(),
            VaultItem::Folder => "folder".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConfigType {
    Editor,
    Conflict,
}

impl ConfigType {
    pub fn to_str(&self) -> &str {
        match self {
            ConfigType::Editor => "editor",
            ConfigType::Conflict => "conflict",
        }
    }
}
