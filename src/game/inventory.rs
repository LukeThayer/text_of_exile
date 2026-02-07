use loot_core::Item;

/// Represents either an item or a currency in the inventory
pub enum InventoryEntry {
    Item(Item),
    Currency { id: String, count: u32 },
}

impl InventoryEntry {
    pub fn as_item(&self) -> Option<&Item> {
        match self {
            InventoryEntry::Item(item) => Some(item),
            InventoryEntry::Currency { .. } => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            InventoryEntry::Item(item) => item.name.clone(),
            InventoryEntry::Currency { id, count } => {
                if *count > 1 {
                    format!("{} x{}", id, count)
                } else {
                    id.clone()
                }
            }
        }
    }

    pub fn is_currency(&self) -> bool {
        matches!(self, InventoryEntry::Currency { .. })
    }

    pub fn rarity(&self) -> &str {
        match self {
            InventoryEntry::Item(item) => &item.rarity,
            InventoryEntry::Currency { .. } => "normal",
        }
    }
}

pub struct Inventory {
    entries: Vec<InventoryEntry>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.entries.push(InventoryEntry::Item(item));
    }

    pub fn add_currency(&mut self, id: String) {
        // Stack currencies
        for entry in &mut self.entries {
            if let InventoryEntry::Currency {
                id: existing_id,
                count,
            } = entry
            {
                if existing_id == &id {
                    *count += 1;
                    return;
                }
            }
        }
        // New currency stack
        self.entries.push(InventoryEntry::Currency { id, count: 1 });
    }

    pub fn get(&self, index: usize) -> Option<&InventoryEntry> {
        self.entries.get(index)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.entries.len() {
            if let InventoryEntry::Item(_) = &self.entries[index] {
                if let InventoryEntry::Item(item) = self.entries.remove(index) {
                    return Some(item);
                }
            }
        }
        None
    }

    /// Replace an item at the given index, keeping it in the same position
    pub fn replace_item(&mut self, index: usize, new_item: Item) -> bool {
        if index < self.entries.len() {
            if let InventoryEntry::Item(_) = &self.entries[index] {
                self.entries[index] = InventoryEntry::Item(new_item);
                return true;
            }
        }
        false
    }

    pub fn remove_currency(&mut self, index: usize) -> Option<String> {
        if index < self.entries.len() {
            if let InventoryEntry::Currency { id, count } = &mut self.entries[index] {
                let currency_id = id.clone();
                if *count > 1 {
                    *count -= 1;
                } else {
                    self.entries.remove(index);
                }
                return Some(currency_id);
            }
        }
        None
    }

    pub fn has_currency(&self) -> bool {
        self.entries
            .iter()
            .any(|e| matches!(e, InventoryEntry::Currency { .. }))
    }

    pub fn get_currency_indices(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e, InventoryEntry::Currency { .. }))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn get_currencies(&self) -> Vec<(usize, &str)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if let InventoryEntry::Currency { id, .. } = e {
                    Some((i, id.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InventoryEntry> {
        self.entries.iter()
    }
}
