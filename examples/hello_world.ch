fn testing() {
   test = "hello world! This is Chorus"
   print(test)
   test()
}

fn test() {
   test = "im a nested func"
   print(test)
}

testing()
testing()
testing()