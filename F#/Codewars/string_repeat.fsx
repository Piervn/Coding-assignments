let repeatStr (n: int) (s: string): string = 
    String.replicate n s

printfn "%A" ("ABABAB" = repeatStr 3 "AB")
printfn "%A" ("" = repeatStr 3 "")
printfn "%A" ("" = repeatStr 0 "*")
