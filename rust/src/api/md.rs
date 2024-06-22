use flutter_rust_bridge::frb;
use markdown::{CompileOptions, Constructs, LineEnding, Options, ParseOptions};

// markdown to html
pub async fn markdow2html(str: String) -> String {
    let html = markdown::to_html(&str);
    html
}

// markdown html for options
pub async fn markdown2html_with_options(
    str: String,
    options: Option<MkCompileOptions>,
    parse_options: Option<MkParseOptions>,
) -> Result<String, String> {
    let mut complie: CompileOptions = CompileOptions::default();
    let mut parse = ParseOptions::default();
    if let Some(v) = options {
        complie = v.to_mk_compile();
    }
    if let Some(p) = parse_options {
        parse = p.to_options();
    }
    let html = markdown::to_html_with_options(
        &str,
        &Options {
            parse: parse,
            compile: complie,
        },
    );
    match html {
        Ok(html_value) => Ok(html_value),
        Err(_) => Err("parse failed".to_string()),
    }
}

//编译选项
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct MkCompileOptions {
    pub allow_dangerous_html: bool,
    pub allow_dangerous_protocol: bool,
    pub gfm_footnote_label: Option<String>,
    pub gfm_footnote_label_tag_name: Option<String>,
    pub gfm_footnote_label_attributes: Option<String>,
    pub gfm_footnote_back_label: Option<String>,
    pub gfm_footnote_clobber_prefix: Option<String>,
    pub gfm_task_list_item_checkable: bool,
    pub gfm_tagfilter: bool,
    pub default_line_ending: Option<MkLineEnding>,
}

impl MkCompileOptions {
    //options to
    fn to_mk_compile(&self) -> CompileOptions {
        CompileOptions {
            allow_dangerous_html: self.allow_dangerous_html,
            allow_dangerous_protocol: self.allow_dangerous_protocol,
            default_line_ending: self.get_line_ending(),
            gfm_footnote_label: self.gfm_footnote_label.clone(),
            gfm_footnote_label_tag_name: self.gfm_footnote_label_tag_name.clone(),
            gfm_footnote_label_attributes: self.gfm_footnote_label_attributes.clone(),
            gfm_footnote_back_label: self.gfm_footnote_back_label.clone(),
            gfm_footnote_clobber_prefix: self.gfm_footnote_clobber_prefix.clone(),
            gfm_task_list_item_checkable: self.gfm_task_list_item_checkable,
            gfm_tagfilter: self.gfm_tagfilter,
        }
    }

    // pub fn default_options() -> MkCompileOptions {
    //     MkCompileOptions {
    //         allow_dangerous_html: false,
    //         allow_dangerous_protocol: false,
    //         gfm_footnote_label: None,
    //         gfm_footnote_label_tag_name: None,
    //         gfm_footnote_label_attributes: None,
    //         gfm_footnote_back_label: None,
    //         gfm_footnote_clobber_prefix: None,
    //         gfm_task_list_item_checkable: false,
    //         gfm_tagfilter: true,
    //         default_line_ending: Some(MkLineEnding::LineFeed),
    //     }
    // }

    fn get_line_ending(&self) -> LineEnding {
        if let Some(v) = &self.default_line_ending {
            return v.to_mk_line_ending();
        }
        LineEnding::LineFeed
    }
}

pub enum MkLineEnding {
    CarriageReturnLineFeed,
    CarriageReturn,
    LineFeed,
}

impl MkLineEnding {
    fn to_mk_line_ending(&self) -> LineEnding {
        match self {
            MkLineEnding::CarriageReturnLineFeed => LineEnding::CarriageReturnLineFeed,
            MkLineEnding::CarriageReturn => LineEnding::CarriageReturn,
            MkLineEnding::LineFeed => LineEnding::LineFeed,
        }
    }
}
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct MkParseOptions {
    pub constructs: MkConstructs,
    pub gfm_strikethrough_single_tilde: bool,
    pub math_text_single_dollar: bool,
}

impl MkParseOptions {
    fn to_options(&self) -> ParseOptions {
        ParseOptions {
            constructs: self.constructs.to_constructs(),
            gfm_strikethrough_single_tilde: self.gfm_strikethrough_single_tilde,
            math_text_single_dollar: self.math_text_single_dollar,
            ..Default::default()
        }
    }
}
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct MkConstructs {
    pub attention: bool,
    pub autolink: bool,
    pub block_quote: bool,
    pub character_escape: bool,
    pub character_reference: bool,
    pub code_indented: bool,
    pub code_fenced: bool,
    pub code_text: bool,
    pub definition: bool,
    pub frontmatter: bool,
    pub gfm_autolink_literal: bool,
    pub gfm_footnote_definition: bool,
    pub gfm_label_start_footnote: bool,
    pub gfm_strikethrough: bool,
    pub gfm_table: bool,
    pub gfm_task_list_item: bool,
    pub hard_break_escape: bool,
    pub hard_break_trailing: bool,
    pub heading_atx: bool,
    pub heading_setext: bool,
    pub html_flow: bool,
    pub html_text: bool,
    pub label_start_image: bool,
    pub label_start_link: bool,
    pub label_end: bool,
    pub list_item: bool,
    pub math_flow: bool,
    pub math_text: bool,
    pub mdx_esm: bool,
    pub mdx_expression_flow: bool,
    pub mdx_expression_text: bool,
    pub mdx_jsx_flow: bool,
    pub mdx_jsx_text: bool,
    pub thematic_break: bool,
}

impl MkConstructs {
    fn to_constructs(&self) -> Constructs {
        Constructs {
            attention: self.attention,
            autolink: self.autolink,
            block_quote: self.block_quote,
            character_escape: self.character_escape,
            character_reference: self.character_reference,
            code_indented: self.code_indented,
            code_fenced: self.code_fenced,
            code_text: self.code_text,
            definition: self.definition,
            frontmatter: self.frontmatter,
            gfm_autolink_literal: self.gfm_autolink_literal,
            gfm_footnote_definition: self.gfm_footnote_definition,
            gfm_label_start_footnote: self.gfm_label_start_footnote,
            gfm_strikethrough: self.gfm_strikethrough,
            gfm_table: self.gfm_table,
            gfm_task_list_item: self.gfm_task_list_item,
            hard_break_escape: self.hard_break_escape,
            hard_break_trailing: self.hard_break_trailing,
            heading_atx: self.heading_atx,
            heading_setext: self.heading_setext,
            html_flow: self.html_flow,
            html_text: self.html_text,
            label_start_image: self.label_start_image,
            label_start_link: self.label_start_link,
            label_end: self.label_end,
            list_item: self.list_item,
            math_flow: self.math_flow,
            math_text: self.math_text,
            mdx_esm: self.mdx_esm,
            mdx_expression_flow: self.mdx_expression_flow,
            mdx_expression_text: self.mdx_expression_text,
            mdx_jsx_flow: self.mdx_jsx_flow,
            mdx_jsx_text: self.mdx_jsx_text,
            thematic_break: self.thematic_break,
        }
    }
}
