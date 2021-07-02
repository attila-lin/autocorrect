// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/php.pest"]
struct PHPParser;

pub fn format_php(text: &str, lint: bool) -> String {
  let result = PHPParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_php_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_php_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::string | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item | Rule::php => {
      for sub in item.into_inner() {
        format_php_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_php() {
    let example = r###"
<div class="container">
  <p>目前html tag里的无法处理</p>
  <?php
  /** 
   * 第1行注释
   * 第2行注释
   */
  class HelloWorld {
      // 这是第3行注释
      var singleLineString: String = "单行string测试"
      var multilineString: String = "多行string测试
      第2行字符串"
  }
  ?>
</div>
"###;

    let expect = r###"
<div class="container">
  <p>目前html tag里的无法处理</p>
  <?php
  /** 
   * 第 1 行注释
   * 第 2 行注释
   */
  class HelloWorld {
      // 这是第 3 行注释
      var singleLineString: String = "单行 string 测试"
      var multilineString: String = "多行 string 测试
      第 2 行字符串"
  }
  ?>
</div>
"###;

    assert_eq!(expect, format_php(example, false));
  }
}
