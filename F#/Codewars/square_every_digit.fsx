open System
let squareDigits (n: int) = 
    n 
    |> string 
    |> Seq.map (string >> int >> (fun x -> x * x) >> string)
    |> String.concat ""
    |> int

let test = squareDigits 12345

printfn "%A" test