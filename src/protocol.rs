use std::error::Error;
use std::fmt;
use std::str::from_utf8;

pub struct Argument {
  pub pos: usize,
  pub len: usize,
}

pub struct Parser<'a> {
  pub data: &'a [u8],
  pub argc: usize,
  pub argv: Vec<Argument>,
}

pub enum ParseError {
  Incomplete,
  BadProtocol,
}

impl fmt::Debug for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}", self.description());
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return self.description().fmt(f);
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    match *self {
      ParseError::Incomplete => "数据不完整",
      ParseError::BadProtocol => "数据无效",
    }
  }
  fn cause(&self) -> Option<&Error> {
    None
  }
}

impl<'a> Parser<'a> {
  pub fn new(data: &[u8], argc: usize, argv: Vec<Argument>) -> Parser {
    return Parser {
      data: data,
      argc: argc,
      argv: argv,
    };
  }

  pub fn get_str(&self, pos: usize) -> Result<&str, i32> {
    if pos >= self.argc {
      return Err(0);
    }
    let arg = &self.argv[pos];
    let res = from_utf8(&self.data[arg.pos..arg.pos + arg.len]);
    if res.is_err() {
      return Err(1);
    }
    return Ok(res.unwrap());
  }
}

impl<'a> fmt::Debug for Parser<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}", self.description());
  }
}

impl<'a> fmt::Display for Parser<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return self.description().fmt(f);
  }
}

impl<'a> Error for Parser<'a> {
  fn description(&self) -> &str {
    return "parser";
  }
  fn cause(&self) -> Option<&Error> {
    None
  }
}

pub fn parse_int(input: &[u8], len: usize) -> Result<(usize, usize), ParseError> {
  let mut i = 0;
  let mut argc = 0;
  while input[i] as char != '\r' {
    let c = input[i] as char;
    if c < '0' || c > '9' {
      return Err(ParseError::BadProtocol);
    }
    argc *= 10;
    argc += input[i] as usize - '0' as usize;
    i += 1;
    if i == len {
      return Err(ParseError::Incomplete);
    }
  }
  i += 1;
  if input[i] as char != '\n' {
    return Err(ParseError::BadProtocol);
  }
  return Ok((argc, i + 1));
}

pub fn parse(input: &[u8], len: usize) -> Result<Parser, ParseError> {
  if input[0] as char != '*' {
    return Err(ParseError::BadProtocol);
  } else {
    let mut pos = 1;
    let (argc, intlen) = parse_int(&input[pos..len], len - pos)?;
    pos += intlen;
    let mut argv = Vec::new();
    for i in 0..argc {
      if input[pos] as char != '$' {
        return Err(ParseError::BadProtocol);
      }
      pos += 1;
      let (arglen, arglenlen) = parse_int(&input[pos..len], len - pos)?;
      pos += arglenlen;
      let arg = Argument {
        pos: pos,
        len: arglen,
      };
      argv.push(arg);
      pos += arglen + 2;
      if pos > len || (pos == len && i != argc - 1) {
        return Err(ParseError::Incomplete);
      }
    }
    return Ok(Parser::new(input, argc, argv));
  }
}
