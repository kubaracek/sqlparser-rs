// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::ast::Statement;
use crate::ast::Statement::{SelectAsStruct, SelectAsValue};
use crate::dialect::Dialect;
use crate::keywords::Keyword;
use crate::parser::{Parser, ParserError};

/// A [`Dialect`] for [Google Bigquery](https://cloud.google.com/bigquery/)
#[derive(Debug, Default)]
pub struct BigQueryDialect;

impl Dialect for BigQueryDialect {

    fn parse_statement(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        // Check if the SELECT statement starts
        if parser.parse_keyword(Keyword::SELECT) {
            // TODO: Either parse using parse_value_table or regular SELECT

            let selection_type = self.parse_value_table(parser);
            // Handle parsing for SELECT AS STRUCT or SELECT AS VALUE
            // match selection_type {
            //     Some(selection_type) => {
            //         self.parse_select_as(parser, selection_type)
            //     }
            //     None => {
            //         // Parse regular SELECT statement here
            //         // You can call your regular SELECT parsing function
            //         unimplemented!("Parsing regular SELECT statement is not implemented yet")
            //     }
            // }
            None
        } else {
            None
        }
    }

    // See https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '`'
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_'
    }
}

impl BigQueryDialect {
    fn parse_value_table(&self, parser: &mut Parser) -> Option<Statement> {
        // Check for AS keyword followed by STRUCT or VALUE
        if parser.parse_keyword(Keyword::AS) {
            if parser.parse_keyword(Keyword::STRUCT) {
                return Some(SelectAsStruct {
                    select_list: vec![] // TODO
                });
            } else if parser.parse_keyword(Keyword::VALUE) {
                return Some(SelectAsValue {
                    select_list: vec![] // TODO
                });
            }
        }
        None
    }
}
