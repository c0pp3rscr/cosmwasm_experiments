## Contract A calls Contract B, which then calls contract C, C calls B again (another function, no recursion) . Reply is enabled. The Aim of the experiment is to see the order of the reply execution. calls ~ submsg
```
  -->   -->  
A     B     C
        <-- 
```

## RESULTS
```
---- integration_tests::tests::multicall::multicall stdout ----
hmm
hmm
hmm
Contract ID contract-A Inside Function A
Contract ID contract-B Inside Function A
Contract ID contract-C Inside Function A
Contract ID contract-B Inside Function B
the contract is contract-C
the contract is contract-B
the contract is contract-A
```
  
