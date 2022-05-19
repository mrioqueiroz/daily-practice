module Main where

data Group = Group { groupSize :: Int , groupName :: String } deriving Show

names :: [String]
names = ["Sheldon", "Leonard", "Penny", "Rajesh", "Howard"]

doubleGroup :: Group -> Group
doubleGroup (Group size name) = Group (2 * size) name

groups :: [Group]
groups = map (Group 1) names ++ map doubleGroup groups

nth :: Int -> [Group] -> String
nth _ [] = error "unreachable"
nth n (Group size name:rest)
  | n <= size = name
  | otherwise = nth (n - size) rest

main :: IO ()
main = undefined
