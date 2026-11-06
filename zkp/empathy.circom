pragma circom 2.1.0;

include "circomlib/circuits/comparators.circom";

template EmpathyProof() {
    signal input empathy;      // 0..10000
    signal input threshold;    // 0..10000
    signal input salt;         // satunnaisuus
    signal output ok;          // 1 jos empathy >= threshold

    component gte = GreaterEqThan(14);
    gte.in[0] <== empathy;
    gte.in[1] <== threshold;
    ok <== gte.out;
}

component main = EmpathyProof();
