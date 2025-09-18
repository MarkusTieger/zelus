/*
 * Copyright (C) 2025 Markus Probst
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use core::ops::{Deref, DerefMut};
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use utoipa::__dev::ComposeSchema;
use utoipa::openapi::{RefOr, Schema};
use utoipa::ToSchema;

#[derive(Debug, Clone)]
pub struct JsonVec<T: Clone>(pub Vec<T>);

impl<T: Clone> Deref for JsonVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for JsonVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + Clone> Serialize for JsonVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_json::to_string(&self.0)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de, T: for<'a> Deserialize<'a> + Clone> Deserialize<'de> for JsonVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::from_str(<&str>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

impl<T: ToSchema + ComposeSchema + Clone> ToSchema for JsonVec<T> {
    fn name() -> Cow<'static, str> {
        <Vec<T>>::name()
    }

    fn schemas(schemas: &mut Vec<(String, RefOr<Schema>)>) {
        <Vec<T>>::schemas(schemas);
    }
}

impl<T: ToSchema + ComposeSchema + Clone> ComposeSchema for JsonVec<T> {
    fn compose(new_generics: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        <Vec<T>>::compose(new_generics)
    }
}
