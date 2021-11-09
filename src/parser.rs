pub mod parser {
  pub use structs::structs::*;

  #[derive(Debug)]
  pub struct Parser {
    pub pos: usize,
    pub last_pos: usize,
    pub current_token: Option<Token>,
    pub next_token: Option<Token>,
    pub tokens: Vec<Token>
  }

  impl Parser {
    // Move on to the next token, return the next token.
    fn next(&mut self) -> Option<Token> {
      if self.pos > self.last_pos { // Means we reached the end.
        // Set everything to None.
        self.current_token = None;
        self.next_token = None;
        return None;
      }
      self.current_token = Some(self.tokens[self.pos].clone()); // Update current_token

      if self.pos + 1 > self.last_pos { // Means we are on the last token.
        // Set next_token to None.
        self.next_token = None;
        self.pos += 1;
        return None;
      }
      self.next_token = Some(self.tokens[self.pos+1].clone()); // Update next_token
      self.pos += 1; // Update our position

      return Some(self.tokens[self.pos].clone()); // Return the next token.
    }

    fn expect(&mut self, typ_tkn: Option<Vec<TokenType>>, typ_str: Option<Vec<String>>) {
      if self.current_token == None {
        self.next();
      } else {
        let current_token = self.current_token.as_ref().unwrap();
        if typ_tkn != None {
          if ! typ_tkn.as_ref().unwrap().contains(&current_token.typ) {
            panic!("Expected one of: {:?} on line {}, instead found {:?}", typ_tkn.clone().unwrap(),
            current_token.line, current_token.typ);
          } else {}
        }
        if typ_str != None {
          if ! typ_str.unwrap().contains(&current_token.value) {
            panic!("Expected one of: {:?} on line {}, instead found {:?}", typ_tkn.clone().unwrap(),
            current_token.line, current_token.value);
          } else {}
        }
        self.next();
      }
    }

    fn skip_newlines(&mut self) {
      while self.current_token.as_ref().unwrap().typ == TokenType::Newline {
        self.next();
      }
    }

    pub fn parse(&mut self) -> Vec<AST> {
      let mut program: Vec<AST> = Vec::new();
      self.next(); // Get the initial tokens, otherwise we would have "None"

      while self.current_token != None { // Loop until no more token.
        // Skip newlines,  I should make this a funtion.
        self.skip_newlines();
        program.push(self.parse_top()); // Call parse top and append result to our program vec.
        self.next(); // Go to next token, repeat the process yeet.
      }
      return program; // Return our AST Vec, sir program.
    }

    fn parse_top(&mut self) -> AST {
      let current = self.current_token.clone().unwrap();
      if current.typ != TokenType::Keyword {
        let expr = self.parse_expr(); // This is an expression, so lettus parse it.
        expr
      } else {
        if ["Integer".to_string(), "Decimal".to_string(), "String".to_string()].contains(&current.value) {
          let a = self.parse_assignment();
          a
        } else if current.value == "funk" {
          self.expect(None, Some(vec!("funk".to_string())));
          self.parse_function()
        } else if current.value == "if" {
          self.parse_conditional()
        }
        else {
          AST::Integer{value: 0}
        }
      }
  }
    

    fn parse_expr(&mut self) -> AST {
      let mut result = self.parse_term();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator &&
      ["+".to_string(), "-".to_string(), "==".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op: &String = &self.current_token.as_ref().unwrap().value.clone();
        self.expect(Some(vec!(TokenType::Operator)), None);
        result = AST::BiOpAST{op: op.to_string(), left: Box::new(result), right: Box::new(self.parse_expr())};
      }
      result
    }

    fn parse_term(&mut self) -> AST {
      let mut result = self.parse_factor();

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Operator && 
      ["*".to_string(), "/".to_string()].contains(&self.current_token.as_ref().unwrap().value) {
        let op = &self.current_token.as_ref().unwrap().value.clone();
        self.expect(Some(vec!(TokenType::Operator)), None);
        result = AST::BiOpAST{op: op.to_string(), left: Box::new(result), right: Box::new(self.parse_term())};
      }
      result
    }

    fn parse_factor(&mut self) -> AST {
      let current = self.current_token.clone().unwrap();

      if current.typ == TokenType::Number { // Handle Numbers
        if current.value.contains('.') {
          let val = current.value.parse::<f64>().unwrap(); // Decimal Numbers
          self.expect(Some(vec!(TokenType::Number)), None);
          AST::Decminal{value: val}
        } else { // Integer Numbers
          let val = current.value.parse::<i64>().unwrap();
          self.expect(Some(vec!(TokenType::Number)), None);
          AST::Integer { value: val }
        }
      }

      else if current.typ == TokenType::String { // Handle Strings
        let val = self.current_token.clone().unwrap().value;
        self.expect(Some(vec!(TokenType::String)), None);
        AST::Str { value: val }
      }

      else if current.typ == TokenType::LPar { // Handle Pars
        self.expect(Some(vec!(TokenType::LPar)), None);
        let result = self.parse_expr();
        self.expect(Some(vec!(TokenType::RPar)), None);
        return result;
      } 

      else if current.typ == TokenType::Identifier {
        let var = current;
        self.expect(Some(vec!(TokenType::Identifier)), None);

        if self.current_token.as_ref().unwrap().typ == TokenType::LPar {
          let a = self.parse_call(var);
          a
        }

        else {
          AST::CallItem{ name: var.value, call_type: String::from("VariableCall"), args: None, scope: String::from("Global") }
        }
        
      }

      else {
        panic!("Unknown token: {:?} on line {}.", self.current_token.as_ref().unwrap(), self.current_token.as_ref().unwrap().line);
      }
    }

    fn parse_call(&mut self, name: Token) -> AST {
      self.expect(Some(vec!(TokenType::LPar)), None);
      let args = self.parse_args();

      AST::CallItem{ name: name.value, call_type: String::from("FunctionCall"), args: Some(args), scope: String::from("Global") }
    }

    fn parse_args(&mut self) -> Vec<Box<AST>> {
      let mut args: Vec<Box<AST>> = vec!();

      while self.current_token != None {
        if [TokenType::RPar, TokenType::Newline, TokenType::Semi].contains(&self.current_token.as_ref().unwrap().typ) {
          self.expect(Some(vec!(TokenType::RPar, TokenType::Newline, TokenType::Semi)), None);
          break;
        }

        args.push(Box::new(self.parse_expr()));
        self.expect(Some(vec!(TokenType::Comma, TokenType::RPar)), None);
      }

      args
    }

    fn parse_assignment(&mut self) -> AST {
      let line = self.current_token.as_ref().unwrap().line;
      self.expect(Some(vec!(TokenType::Keyword)), None);

      let var_name = self.current_token.clone().unwrap().value;
      self.expect(Some(vec!(TokenType::Identifier)), None);
      self.expect(None, Some(vec!("=".to_string())));
    
      let value: AST = self.parse_expr();
      self.expect(Some(vec!(TokenType::Semi, TokenType::Newline)), None);

      AST::Assgignment { name: var_name, value: Box::new(value), scope: "Global".to_string(), line: line }
    }

    fn parse_function(&mut self) -> AST {
      let expected_return = self.current_token.clone().unwrap().value;
      self.expect(None, Some(vec!("Integer".to_string(), "Decimal".to_string(), "String".to_string())));

      let funk_name = self.current_token.clone().unwrap().value;
      self.expect(Some(vec!(TokenType::Identifier)), None);

      let params: Vec<Box<AST>> = self.parse_params();
      let body: Vec<Box<AST>> = self.parse_body();

      AST::Funktion { name: funk_name, return_typ: expected_return, params: params, body: body }
    }

    fn parse_params(&mut self) -> Vec<Box<AST>> {
      let mut params: Vec<Box<AST>> = vec!();
      self.expect(Some(vec!(TokenType::LPar)), None);

      while self.current_token != None {
        if self.current_token.as_ref().unwrap().typ == TokenType::RPar {
          self.expect(Some(vec!(TokenType::RPar)), None);
          break;
        }
        else if self.current_token.as_ref().unwrap().typ == TokenType::LCurl {
          break;
        }

        let param_typ = self.current_token.clone().unwrap().value;
        self.expect(None, Some(vec!("Integer".to_string(), "Decimal".to_string(), "String".to_string())));
        let param_name = self.current_token.clone().unwrap().value;
        self.expect(Some(vec!(TokenType::Identifier)), None);
        self.expect(Some(vec!(TokenType::Comma, TokenType::RPar)), None);
        params.push(Box::new(AST::FunktionParameter { name: param_name, typ: param_typ }))
      }
      params
    }

    fn parse_body(&mut self) -> Vec<Box<AST>> {
      self.expect(Some(vec!(TokenType::LCurl)), None);
      let mut body: Vec<Box<AST>> = vec!();

      while self.current_token != None && self.current_token.as_ref().unwrap().typ != TokenType::RCurl {
        self.skip_newlines();
        if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::RCurl {
          break;
        }

        body.push(Box::new(self.parse_top()));
      }
      self.expect(Some(vec!(TokenType::RCurl)), None);
      body
    }

    fn parse_conditional(&mut self) -> AST {
      let typ = self.current_token.clone();
      self.expect(None, Some(vec!(typ.clone().unwrap().value)));

      if typ.as_ref().unwrap().value == "else" {
        return AST::Conditional{ typ: typ.clone().unwrap().value, body: self.parse_body(), expr: None, other: None };
      }

      let expr = Some(Box::new(self.parse_expr()));
      let body = self.parse_body();
      let mut other: Option<Box<AST>> = None;

      if self.current_token != None && self.current_token.as_ref().unwrap().typ == TokenType::Keyword &&
      ["elseif".to_string(), "else".to_string()].contains(&self.current_token.clone().unwrap().value) {
        other = Some(Box::new(self.parse_conditional()));
      }

      AST::Conditional{ typ: typ.clone().unwrap().value, body: body, expr: expr, other: other }
    }
  }
}