use crate::prelude::*;

/// Editor trait
/// provides egui widgets for implementing types
pub trait Editor {
    /// adds an egui widget to the ui
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String);
}

///////////////////////////////////////////////////////////////////
// primitive types

impl Editor for bool {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.checkbox(self, "Checked");
    }
}

impl Editor for f32 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(0.1));
    }
}

impl Editor for String {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.text_edit_singleline(self);
    }
}
impl Editor for FixedString<32> {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.text_edit_singleline(&mut self.0);
    }
}
impl Editor for FixedString<256> {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.text_edit_multiline(&mut self.0);
    }
}

// integer types
// todo refactor with num crate (some traits for integers)?

impl Editor for u8 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}
impl Editor for u16 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}
impl Editor for u32 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}
impl Editor for u64 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}

impl Editor for i8 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}
impl Editor for i16 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}
impl Editor for i32 {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        ui.add(egui::DragValue::new(self).speed(1));
    }
}

// vectors
impl<T> Editor for Vec<T>
where
    T: Editor + std::default::Default,
{
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        ui.vertical(|ui| {
            // add and remove buttons
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    self.push(T::default());
                };
                if ui.button("Remove").clicked() {
                    // remove last (todo)
                    if self.len() > 1 {
                        self.remove(self.len() - 1);
                    }
                };
            });

            // the vector, allowed to panic here since this always needs a prop name
            ui.push_id(name.clone(), |ui| {
                egui::CollapsingHeader::new("elements").show(ui, |ui| {
                    for (i, element) in self.iter_mut().enumerate() {
                        element.add_editor(ui, format!("{}.{}", name, i));
                    }
                });
            });
        });
    }
}

impl<const N: usize> Editor for [AttributeId; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [FactionRequirement; 10] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [SkillId; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [EffectId; 4] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}

// colors
impl Editor for [u8; 4] {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut color: ecolor::Color32 = ecolor::Color32::from_rgba_premultiplied(self[0], self[1], self[2], self[3]);
        ui.color_edit_button_srgba(&mut color);
        self[0] = color.r();
        self[1] = color.g();
        self[2] = color.b();
        self[3] = color.a();
    }
}

// slices
impl Editor for [u8; 3] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [u8; 8] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [u8; 9] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [u8; 16] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [u8; 27] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}

impl<const N: usize> Editor for [i8; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [i32; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [u16; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [f32; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}

impl<const N: usize> Editor for [[u16; 16]; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [[u8; 9]; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [[i8; 3]; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [[u8; 3]; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl<const N: usize> Editor for [[i8; 65]; N] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}

impl Editor for [[[i8; 3]; 65]; 65] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}
impl Editor for [[[u8; 3]; 65]; 65] {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        add_slice_editor(self, ui, name);
    }
}

fn add_slice_editor<T: Editor, const N: usize>(slice: &mut [T; N], ui: &mut egui::Ui, name: String) {
    // the vector, allowed to panic here since this always needs a prop name
    ui.push_id(name.clone(), |ui| {
        egui::CollapsingHeader::new("elements").show(ui, |ui| {
            ui.vertical(|ui| {
                for (i, element) in slice.iter_mut().enumerate() {
                    element.add_editor(ui, format!("{}.{}", name, i));
                }
            });
        });
    });
}

// TODO refactor hashmaps
impl<S> Editor for HashMap<u32, S>
where
    S: Editor,
{
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        ui.vertical(|ui| {
            // add and remove buttons
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    // todo
                };
                if ui.button("Remove").clicked() {
                    // todo
                };
            });

            // the vector, allowed to panic here since this always needs a prop name
            ui.push_id(name.clone(), |ui| {
                egui::CollapsingHeader::new("elements").show(ui, |ui| {
                    for (key, element) in self {
                        ui.horizontal(|ui| {
                            ui.label(key.to_string());
                            element.add_editor(ui, format!("{}.{}", name, key));
                        });
                    }
                });
            });
        });
    }
}

impl<S> Editor for HashMap<u64, S>
where
    S: Editor,
{
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        ui.vertical(|ui| {
            // add and remove buttons
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    // todo
                };
                if ui.button("Remove").clicked() {
                    // todo
                };
            });

            // the vector
            ui.push_id(name.clone(), |ui| {
                egui::CollapsingHeader::new("elements").show(ui, |ui| {
                    for (key, element) in self {
                        ui.horizontal(|ui| {
                            ui.label(key.to_string());
                            element.add_editor(ui, format!("{}.{}", name, key));
                        });
                    }
                });
            });
        });
    }
}

impl<S> Editor for HashMap<(u32, u32), S>
where
    S: Editor,
{
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        ui.vertical(|ui| {
            // add and remove buttons
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    // todo
                };
                if ui.button("Remove").clicked() {
                    // todo
                };
            });

            // the vector
            ui.push_id(format!("{}.ch", name), |ui| {
                egui::CollapsingHeader::new("elements").show(ui, |ui| {
                    for (i, (key, element)) in self.iter_mut().enumerate() {
                        ui.push_id(format!("{}.h.{}", name, i), |ui| {
                            ui.horizontal(|ui| {
                                let label = format!("{},{}", key.0, key.1);
                                ui.label(label);
                                element.add_editor(ui, format!("{}.{}", name, i));
                            });
                        });
                    }
                });
            });
        });
    }
}

// tuples
impl<S: Editor, T: Editor> Editor for (S, T) {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        self.0.add_editor(ui, format!("{}.{}", name, 0));
        self.1.add_editor(ui, format!("{}.{}", name, 1));
    }
}
impl<S: Editor, T: Editor, U: Editor> Editor for (S, T, U) {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        self.0.add_editor(ui, format!("{}.{}", name, 0));
        self.1.add_editor(ui, format!("{}.{}", name, 1));
        self.2.add_editor(ui, format!("{}.{}", name, 2));
    }
}

// optionals
impl<T> Editor for Option<T>
where
    T: Editor + std::default::Default,
{
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        if let Some(value) = self {
            // if it's Some we simply display the editor
            value.add_editor(ui, name);
        } else {
            // if it's None we display a button to create new object
            if ui.button("New").clicked() {
                *self = Some(T::default());
            }
        }
    }
}

// boxes
// check this!!
impl<T: Editor> Editor for Box<T> {
    fn add_editor(&mut self, ui: &mut egui::Ui, name: String) {
        let x = self.as_mut();
        x.add_editor(ui, name);
    }
}

///////////////////////////////////////////////////////////////////
// flags
// todo: refactor into macro

impl Editor for ObjectFlags {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut value = self.bits();
        ui.add(egui::DragValue::new(&mut value).speed(1));
        if let Some(v) = ObjectFlags::from_bits(value) {
            *self = v;
        }
    }
}
impl Editor for LandscapeFlags {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut value = self.bits();
        ui.add(egui::DragValue::new(&mut value).speed(1));
        if let Some(v) = LandscapeFlags::from_bits(value) {
            *self = v;
        }
    }
}
impl Editor for CellFlags {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut value = self.bits();
        ui.add(egui::DragValue::new(&mut value).speed(1));
        if let Some(v) = CellFlags::from_bits(value) {
            *self = v;
        }
    }
}

///////////////////////////////////////////////////////////////////
// missing editor impls for variant enums

impl Editor for AiPackage {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut selected = self.to_owned();
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, AiPackage::Travel(AiTravelPackage::default()), "Travel");
                ui.selectable_value(&mut selected, AiPackage::Wander(AiWanderPackage::default()), "Wander");
                ui.selectable_value(&mut selected, AiPackage::Escort(AiEscortPackage::default()), "Escort");
                ui.selectable_value(&mut selected, AiPackage::Follow(AiFollowPackage::default()), "Follow");
                ui.selectable_value(&mut selected, AiPackage::Activate(AiActivatePackage::default()), "Activate");
            });
        *self = selected;
    }
}

impl Editor for FilterValue {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut selected = self.to_owned();
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, FilterValue::Float(0_f32), "Float");
                ui.selectable_value(&mut selected, FilterValue::Integer(0), "Integer");
            });
        *self = selected;
    }
}

impl Editor for GameSettingValue {
    fn add_editor(&mut self, ui: &mut egui::Ui, _name: String) {
        let mut selected = self.to_owned();
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, GameSettingValue::Float(0_f32), "Float");
                ui.selectable_value(&mut selected, GameSettingValue::Integer(0), "Integer");
                ui.selectable_value(&mut selected, GameSettingValue::String("".to_owned()), "String");
            });
        *self = selected;
    }
}
