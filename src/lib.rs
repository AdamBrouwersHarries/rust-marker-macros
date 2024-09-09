#![allow(unused_variables, dead_code, non_camel_case_types)]

use serde::{de::DeserializeOwned, Serialize};

pub enum MarkerSchema_Location {
    MarkerChart = 0,
    MarkerTable = 1,
    TimelineOverview = 2,
    TimelineMemory = 3,
    TimelineIPC = 4,
    TimelineFileIO = 5,
    StackChart = 6,
}

/// Marker locations to be displayed in the profiler front-end.
pub type Location = MarkerSchema_Location;

pub enum MarkerSchema_Format {
    Url = 0,
    FilePath = 1,
    SanitizedString = 2,
    String = 3,
    UniqueString = 4,
    Duration = 5,
    Time = 6,
    Seconds = 7,
    Milliseconds = 8,
    Microseconds = 9,
    Nanoseconds = 10,
    Bytes = 11,
    Percentage = 12,
    Integer = 13,
    Decimal = 14,
}

/// Formats of marker properties for profiler front-end.
pub type Format = MarkerSchema_Format;

pub enum MarkerSchema_Searchable {
    NotSearchable = 0,
    Searchable = 1,
}

/// Whether it's searchable or not in the profiler front-end.
pub type Searchable = MarkerSchema_Searchable;

/// This object collects all the information necessary to stream the JSON schema
/// that informs the front-end how to display a type of markers.
/// It will be created and populated in `marker_type_display()` functions in each
/// marker type definition, see add/set functions.
///
/// It's a RAII object that constructs and destroys a C++ MarkerSchema object
/// pointed to a specified reference.
pub struct MarkerSchema {
    pub(crate) pin: u32,
}

impl MarkerSchema {
    // Initialize a marker schema with the given `Location`s.
    pub fn new(locations: &[Location]) -> Self {
        MarkerSchema { pin: 42 }
    }

    /// Marker schema for types that have special frontend handling.
    /// Nothing else should be set in this case.
    pub fn new_with_special_frontend_location() -> Self {
        MarkerSchema { pin: 42 }
    }

    /// Optional label in the marker chart.
    /// If not provided, the marker "name" will be used. The given string
    /// can contain element keys in braces to include data elements streamed by
    /// `stream_json_marker_data()`. E.g.: "This is {marker.data.text}"
    pub fn set_chart_label(&mut self, label: &str) -> &mut Self {
        self
    }

    /// Optional label in the marker chart tooltip.
    /// If not provided, the marker "name" will be used. The given string
    /// can contain element keys in braces to include data elements streamed by
    /// `stream_json_marker_data()`. E.g.: "This is {marker.data.text}"
    pub fn set_tooltip_label(&mut self, label: &str) -> &mut Self {
        self
    }

    /// Optional label in the marker table.
    /// If not provided, the marker "name" will be used. The given string
    /// can contain element keys in braces to include data elements streamed by
    /// `stream_json_marker_data()`. E.g.: "This is {marker.data.text}"
    pub fn set_table_label(&mut self, label: &str) -> &mut Self {
        self
    }

    /// Set all marker chart / marker tooltip / marker table labels with the same text.
    /// Same as the individual methods, the given string can contain element keys
    /// in braces to include data elements streamed by `stream_json_marker_data()`.
    /// E.g.: "This is {marker.data.text}"
    pub fn set_all_labels(&mut self, label: &str) -> &mut Self {
        self
    }

    // Each data element that is streamed by `stream_json_marker_data()` can be
    // displayed as indicated by using one of the `add_...` function below.
    // Each `add...` will add a line in the full marker description. Parameters:
    // - `key`: Element property name as streamed by `stream_json_marker_data()`.
    // - `label`: Optional label. Defaults to the key name.
    // - `format`: How to format the data element value, see `Format` above.
    // - `searchable`: Optional, indicates if the value is used in searches,
    //   defaults to false.

    /// Add a key / format row for the marker data element.
    /// - `key`: Element property name as streamed by `stream_json_marker_data()`.
    /// - `format`: How to format the data element value, see `Format` above.
    pub fn add_key_format(&mut self, key: &str, format: Format) -> &mut Self {
        self
    }

    /// Add a key / label / format row for the marker data element.
    /// - `key`: Element property name as streamed by `stream_json_marker_data()`.
    /// - `label`: Optional label. Defaults to the key name.
    /// - `format`: How to format the data element value, see `Format` above.
    pub fn add_key_label_format(&mut self, key: &str, label: &str, format: Format) -> &mut Self {
        self
    }

    /// Add a key / format / searchable row for the marker data element.
    /// - `key`: Element property name as streamed by `stream_json_marker_data()`.
    /// - `format`: How to format the data element value, see `Format` above.
    pub fn add_key_format_searchable(
        &mut self,
        key: &str,
        format: Format,
        searchable: Searchable,
    ) -> &mut Self {
        self
    }

    /// Add a key / label / format / searchable row for the marker data element.
    /// - `key`: Element property name as streamed by `stream_json_marker_data()`.
    /// - `label`: Optional label. Defaults to the key name.
    /// - `format`: How to format the data element value, see `Format` above.
    /// - `searchable`: Optional, indicates if the value is used in searches,
    ///   defaults to false.
    pub fn add_key_label_format_searchable(
        &mut self,
        key: &str,
        label: &str,
        format: Format,
        searchable: Searchable,
    ) -> &mut Self {
        self
    }

    /// Add a key / value static row.
    /// - `key`: Element property name as streamed by `stream_json_marker_data()`.
    /// - `value`: Static value to display.
    pub fn add_static_label_value(&mut self, label: &str, value: &str) -> &mut Self {
        self
    }
}

pub type SpliceableJSONWriter = str;

#[derive(Debug)]
pub struct JSONWriter<'a>(&'a mut SpliceableJSONWriter);

impl<'a> JSONWriter<'a> {
    /// Constructor for the JSONWriter object. It takes a C++ SpliceableJSONWriter
    /// reference as its argument and stores it for later accesses.
    pub(crate) fn new(json_writer: &'a mut SpliceableJSONWriter) -> Self {
        JSONWriter(json_writer)
    }

    /// Adds an int property to the JSON.
    /// Prints: "<name>": <value>
    pub fn int_property(&mut self, name: &str, value: i64) {
        unimplemented!()
    }

    /// Adds a float property to the JSON.
    /// Prints: "<name>": <value>
    pub fn float_property(&mut self, name: &str, value: f64) {
        unimplemented!()
    }

    /// Adds an bool property to the JSON.
    /// Prints: "<name>": <value>
    pub fn bool_property(&mut self, name: &str, value: bool) {
        unimplemented!()
    }

    /// Adds a string property to the JSON.
    /// Prints: "<name>": "<value>"
    pub fn string_property(&mut self, name: &str, value: &str) {
        unimplemented!()
    }

    /// Adds a unique string property to the JSON.
    /// Prints: "<name>": <string_table_index>
    pub fn unique_string_property(&mut self, name: &str, value: &str) {
        unimplemented!()
    }

    /// Adds a null property to the JSON.
    /// Prints: "<name>": null
    pub fn null_property(&mut self, name: &str) {
        unimplemented!()
    }
}

pub trait ProfilerMarker: Serialize + DeserializeOwned {
    /// A static method that returns the name of the marker type.
    fn marker_type_name() -> &'static str;
    /// A static method that returns a `MarkerSchema`, which contains all the
    /// information needed to stream the display schema associated with a
    /// marker type.
    fn marker_type_display() -> MarkerSchema;
    /// A method that streams the marker payload data as JSON object properties.
    /// Please see the [JSONWriter] struct to see its methods.
    fn stream_json_marker_data(&self, json_writer: &mut JSONWriter);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ProfilerMarker;
    use profiler_macros::ProfilerMarker;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize, ProfilerMarker)]
    #[marker_display(MarkerChart, MarkerTable, TimelineIPC)]
    pub struct ExampleMarker {
        #[searchable]
        #[format(Integer)]
        field1: u32,
        #[format(String)]
        field2: String,
        #[format(Integer)]
        field3: std::option::Option<f32>,
    }
}
