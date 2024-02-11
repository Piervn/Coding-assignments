let highestRank (array: int[]) =
    array |> Array.countBy id |> Array.sortByDescending fst |> Array.maxBy snd |> fst

let sndHighestRank = Array.countBy id >> Array.sortByDescending fst >> Array.maxBy snd >> fst

let thirdHighestRank = Array.countBy id >> Array.maxBy (fun (n, c) -> (c, n)) >> fst

let test1 = [|12; 10; 8; 12; 7; 6; 4; 10; 12|]
let test2 = [|12; 10; 8; 12; 7; 6; 4; 10; 12; 10|]
let test3 = [|12; 10; 8; 8; 3; 3; 3; 3; 2; 4; 10; 12; 10|]

let res1 = test1 |> highestRank
let res2 = test2 |> highestRank
let res3 = test3 |> highestRank
let res4 = test1 |> sndHighestRank
let res5 = test2 |> sndHighestRank
let res6 = test3 |> sndHighestRank
let res7 = test1 |> thirdHighestRank
let res8 = test2 |> thirdHighestRank
let res9 = test3 |> thirdHighestRank


res1 |> printfn "%A"
res2 |> printfn "%A"
res3 |> printfn "%A"
res4 |> printfn "%A"
res5 |> printfn "%A"
res6 |> printfn "%A"
res7 |> printfn "%A"
res8 |> printfn "%A"
res9 |> printfn "%A"
