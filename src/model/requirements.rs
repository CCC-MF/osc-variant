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

use crate::model::data_catalogue::DataCatalogue;
use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::Listable;

#[allow(clippy::enum_variant_names)]
pub enum Requirement<'a> {
    PropertyCatalogue(&'a PropertyCatalogue),
    DataCatalogue(&'a DataCatalogue),
    ExternalPropertyCatalogue(String),
    ExternalDataCatalogue(String),
}

impl ToString for Requirement<'_> {
    fn to_string(&self) -> String {
        match self {
            Requirement::PropertyCatalogue(item) => item.to_listed_string(),
            Requirement::DataCatalogue(item) => item.to_listed_string(),
            Requirement::ExternalPropertyCatalogue(name) => {
                format!("Merkmalskatalog (-) '{}' - hier nicht enthalten", name)
            }
            Requirement::ExternalDataCatalogue(name) => {
                format!("Datenkatalog (-) '{}' - hier nicht enthalten", name)
            }
        }
    }
}

pub trait Requires {
    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement>;

    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor) -> String;
}