# Secrets

## Secrets in Flash
```
0x1004_2000┌───────────────────────────┐
           │Frame Key (16B)            │
           │Subscription Key (16B)     │
           │                           │
0x1004_4000├───────────────────────────┤
           │Channel 0 Subscription     │
           │Enable Magic 0x53 (16B)    │
           │Start Timestamp (8B)       │
           │End Timestamp (8B)         │
           │Channel Secret (32B)       │
           │                           │
0x1004_6000├───────────────────────────┤
           │Channel 1 Subscription     │
           │                           │
0x1004_8000├───────────────────────────┤
           │Channel 2 Subscription     │
           │                           │
0x1004_A000├───────────────────────────┤
           │Channel 3 Subscription     │
           │                           │
0x1004_C000├───────────────────────────┤
           │Channel 4 Subscription     │
           │                           │
0x1004_E000├───────────────────────────┤
           │Channel 5 Subscription     │
           │                           │
0x1005_0000├───────────────────────────┤
           │Channel 6 Subscription     │
           │                           │
0x1005_2000├───────────────────────────┤
           │Channel 7 Subscription     │
           │                           │
0x1005_4000├───────────────────────────┤
           │Channel 8 Subscription     │
           │                           │
0x1005_6000└───────────────────────────┘
```