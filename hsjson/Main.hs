module Main where

import Control.Applicative
import Data.Char

-- Define AST.
data JsonValue
  = JsonNull
  | JsonBool Bool -- Encapsulate the boolean type of Haskell.
  | JsonNumber Integer -- No support for floats.
  | JsonString String
  | JsonArray [JsonValue] -- The type becomes recursive.
  | JsonObject [(String, JsonValue)] -- Associative array.
  deriving (Show, Eq)

-- Define parser.
-- NOTE: No proper error reporting.
-- Is parameterized by the thing it parses.
newtype Parser a = Parser
  -- runParser is a field, but it generates a function.
  { runParser :: String -> Maybe (String, a)
  }

-- Prove that the Parser is a Functor.
instance Functor Parser where
  fmap f (Parser p) = Parser $ \input -> do
    (input', x) <- p input
    Just (input', f x)

-- Prove that the Parser is a Applicative.
instance Applicative Parser where
  pure x = Parser $ \input -> Just (input, x)
  (Parser p1) <*> (Parser p2) = Parser $ \input -> do
    (input', f) <- p1 input
    (input'', a) <- p2 input'
    Just (input'', f a)

-- Prove that the Parser is a Alternative.
instance Alternative Parser where
  empty = Parser $ const Nothing
  (Parser p1) <|> (Parser p2) =
    Parser $ \input -> p1 input <|> p2 input

-- Parse a single character.
charP :: Char -> Parser Char
charP x = Parser f
  where
    f (y : ys)
      | y == x = Just (ys, x)
      | otherwise = Nothing
    f [] = Nothing

stringP :: String -> Parser String
stringP = traverse charP

-- Just need to parse a sequence of characters ("null").
jsonNull :: Parser JsonValue
jsonNull = JsonNull <$ stringP "null"

jsonBool :: Parser JsonValue
jsonBool = f <$> (stringP "true" <|> stringP "false")
  where
    f "true" = JsonBool True
    f "false" = JsonBool False
    -- Should never happen.
    f _ = undefined

spanP :: (Char -> Bool) -> Parser String
spanP f = Parser $ \input ->
  let (token, rest) = span f input
   in Just (rest, token)

-- Parser combinator.
notNull :: Parser [a] -> Parser [a]
notNull (Parser p) =
  Parser $ \input -> do
    (input', xs) <- p input
    if null xs
      then Nothing
      else Just (input', xs)

jsonNumber :: Parser JsonValue
jsonNumber = f <$> notNull (spanP isDigit)
  where
    f ds = JsonNumber $ read ds

stringLiteral :: Parser String
stringLiteral = spanP (/= '"')

-- No support for scaping.
jsonString :: Parser JsonValue
jsonString = JsonString <$> (charP '"' *> stringLiteral <* charP '"')

ws :: Parser String
ws = spanP isSpace

sepBy :: Parser a -> Parser b -> Parser [b]
sepBy sep element = (:) <$> element <*> many (sep *> element) <|> pure []

jsonArray :: Parser JsonValue
jsonArray = JsonArray <$> (charP '[' *> ws *> elements <* ws <* charP ']')
  where
    elements = sepBy (ws *> charP ',' <* ws) jsonValue

jsonValue :: Parser JsonValue
jsonValue =
  jsonNull
    <|> jsonBool
    <|> jsonNumber
    <|> jsonString

main :: IO ()
main = undefined
