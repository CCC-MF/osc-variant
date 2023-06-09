/*
 * MIT License
 *
 * Copyright (c) 2023 Comprehensive Cancer Center Mainfranken
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::profile::{FormReference, Profile};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub mod data_catalogue;
pub mod data_form;
pub mod onkostar_editor;
pub mod property_catalogue;
pub mod unterformular;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Script {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Valid")]
    valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlausibilityRule<T> {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Bezeichnung")]
    bezeichnung: String,
    #[serde(rename = "Formula")]
    formula: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Editable")]
    editable: bool,
    #[serde(rename = "Valid")]
    valid: bool,
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(rename = "ConditionValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_valid: Option<bool>,
    #[serde(rename = "Statusauswirkung")]
    statusauswirkung: String,
    #[serde(rename = "DataFormEntries")]
    data_form_entries: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Entries<T> {
    #[serde(rename = "Entry")]
    entry: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ansicht {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Konfiguration")]
    konfiguration: String,
    #[serde(rename = "DataForm")]
    data_form: String,
    #[serde(rename = "DataCatalogue")]
    data_catalogue: String,
    #[serde(rename = "TypAuswahl")]
    typ_auswahl: String,
    #[serde(rename = "Suche")]
    suche: bool,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "InBibliothekAusliefern")]
    in_bibliothek_ausliefern: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ansichten {
    #[serde(rename = "Ansicht", default)]
    program_module: Vec<Ansicht>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MenuCategory {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "position")]
    position: String,
    #[serde(rename = "column")]
    column: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    #[serde(rename = "Condition")]
    condition: String,
    #[serde(rename = "Valid")]
    valid: bool,
    #[serde(rename = "RefEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_entries: Option<RefEntries>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RefEntries {
    #[serde(rename = "RefEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_entry: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlausibilityRules<T> {
    #[serde(rename = "PlausibilityRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    plausibility_rule: Option<Vec<PlausibilityRule<T>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Haeufigkeiten {
    #[serde(rename = "Haeufigkeit", default)]
    haeufigkeit: Vec<Haeufigkeit>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Haeufigkeit {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Notiz")]
    notiz: String,
    #[serde(rename = "Status")]
    status: bool,
    #[serde(rename = "Formel")]
    formel: String,
    #[serde(rename = "Analysezweck")]
    analysezweck: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Ueberschrift")]
    ueberschrift: String,
    #[serde(rename = "TaeglichAktualisieren")]
    taeglich_aktualisieren: bool,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "TabellenName")]
    tabellen_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Bibliothek {
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ordner {
    #[serde(rename = "Bibliothek")]
    bibliothek: Bibliothek,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "ParentOrdner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_order: Option<Box<Ordner>>,
}

fn apply_profile_to_form_entry<E>(entry: &mut E, form_reference: &FormReference)
where
    E: FormEntry,
{
    if entry.get_type() == "formReference" && entry.get_name() == form_reference.name {
        if let Some(profile_referenced_data_form) = &form_reference.referenced_data_form {
            entry.update_referenced_data_form(profile_referenced_data_form.clone());
        }
        if let Some(profile_anzeige) = &form_reference.anzeige {
            entry.update_anzeige(profile_anzeige.clone());
        }
        if let Some(profile_anzeige_auswahl) = &form_reference.anzeige_auswahl {
            entry.update_anzeige_auswahl(profile_anzeige_auswahl.clone());
        }
        if let Some(scripts_code) = &form_reference.escaped_scripts_code() {
            entry.update_scripts_code(scripts_code.clone());
        }
    }
}

pub trait FormEntryContainer {
    fn apply_profile(&mut self, profile: &Profile);
}

pub trait Listable {
    fn to_listed_string(&self) -> String;
}

pub trait Sortable {
    fn sorting_key(&self) -> String;
}

pub trait Comparable: Debug {
    fn get_name(&self) -> String;
    fn get_revision(&self) -> u16;
    fn get_hash(&self) -> String {
        let mut h = DefaultHasher::new();
        format!("{:?}", self).hash(&mut h);
        h.finish().to_string()
    }
}

pub trait FormEntry {
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn update_referenced_data_form(&mut self, value: String);
    fn update_anzeige(&mut self, value: String);
    fn update_anzeige_auswahl(&mut self, value: String);
    fn update_scripts_code(&mut self, value: String);
}
