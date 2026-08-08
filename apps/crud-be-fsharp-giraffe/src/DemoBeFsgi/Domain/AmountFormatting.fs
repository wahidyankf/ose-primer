module DemoBeFsgi.Domain.AmountFormatting

open System.Globalization

let formatAmount (currency: string) (amount: decimal) =
    match currency.ToUpperInvariant() with
    | "IDR" -> amount.ToString("0", CultureInfo.InvariantCulture)
    | _ -> amount.ToString("0.00", CultureInfo.InvariantCulture)
