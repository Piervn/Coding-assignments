let digPow (n: int) (p: int) =
  let sum = 
    n
    |> string
    |> Seq.map (string >> int64) 
    |> Seq.mapi (fun i c -> pown c (p + i) ) 
    |> Seq.sum 
  let N = int64 n
  if sum % N = 0L then (sum / N) else -1L

printfn "%A" (digPow 89 1)
printfn "%A" (digPow 92 1)
printfn "%A" (digPow 46288 3)
printfn "%A" (digPow 114 3)
