let digPow (n: int) (p: int) =
  let sum = n |> string |> Seq.map (fun c -> int c - int '0') |> Seq.mapi (fun i x -> float x ** (float (p + i))) |> Seq.sum
  if sum % float n = 0.0 then (sum / float n) else -1

printfn "%A" (digPow 89 1)
printfn "%A" (digPow 92 1)
printfn "%A" (digPow 46288 3)
printfn "%A" (digPow 114 3)