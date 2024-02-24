open System
let isPalindrom (s: string) = 
    (s |> Seq.map Char.ToLower |> Seq.toArray |> string) = (s |> Seq.rev |> Seq.map Char.ToLower |> Seq.toArray |> string)

let test = isPalindrom "Abba"
printfn "%A" test