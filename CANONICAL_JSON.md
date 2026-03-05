# Representações Canônicas JSON

Este documento especifica as representações JSON do sistema `mixnet-rust` com conformidade obrigatória a **RFC 8785** para todas operações criptográficas (hash e assinatura).

## RFC 8785: Canonicalização

Todas operações de hash/assinatura **DEVEM** usar:
- Chaves em **ordem alfabética**
- **Sem espaços** (compacto)
- **Sem quebras de linha**

## Tipos (codificação)
- `TrackingCode`, `Element`, `Scalar`: string hexadecimal **maiúscula** (sem `0x`)
- `timestamp`: RFC3339 (ex.: `"2026-03-05T02:09:25.467237740+00:00"`)
- `verifying_key`: DER hexadecimal maiúscula

## Arquivos emitidos

### `config/election_config.json`
```json
{"contests":[{"contest_id":0,"name":"contest_0","num_choices":5}],"crypto":{"h":"026FA250...","h_list":["0229D700...","02D92AF5..."]}}
```

### `outputs/rdv_prime.json`
```json
{"entries":[{"choice":3,"contest":0},{"choice":1,"contest":1}]}
```

### `outputs/rdcv.json`
```json
{"entries":[{"committed_votes":["02B90AFF..."],"timestamp":"2026-03-05T01:59:39...","tracking_code":"C3FF3E7B..."}],"head":"D7442C69...","tail":"A9C8563B..."}
```

### `outputs/rdcv_prime.json`
```json
{"entries":["037F3F53...","03AAA787..."]}
```

### `outputs/zkp_output.json`
```json
{"m_list":["00000000..."],"r_list":["4E3C9357..."],"shuffle_proof":{...},"verifying_key":"3059301306..."}
```

### `outputs/*.sig`
```json
"33F94C55F91935E1662299012E0AC8891C9907BDA7F12D9C7FAFA46CE373FCD2800961A333184FDA090704DE9C0094A9E9BDD6D1663556AC1C0BEF127E7C901C"
```

## Entradas de hash

### Tracking code (`E2Easy::vote`)
```json
["A9C8563BF45F...","2026-03-05T02:09:25.467237740+00:00",["03EFCDAB4451...","0282BAF46ED1..."]]
```

Tupla com 3 elementos: `(prev_tracking_code, timestamp, committed_votes)`.

### Head/CLOSE (`E2Easy::tally`)
```json
["A9C8563BF45F...","CLOSE"]
```

Tupla com 2 elementos: `(prev_tracking_code, "CLOSE")`.