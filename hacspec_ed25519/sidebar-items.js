initSidebarItems({"enum":[["Error",""]],"fn":[["alg2_verify","Algorithm 2 from https://eprint.iacr.org/2020/1244.pdf. Cofactored verification. Rejects non-canonical encoding of points. Rejects small-order points for the public key."],["alg3_batch_verify","Batch verification. Algorithm 3 from https://eprint.iacr.org/2020/1244.pdf. Cofactored verification. Rejects non-canonical encoding of points. Rejects small-order points for the public key."],["ietf_cofactored_batch_verify","Batch verification. Cofactored verification. Rejects non-canonical encoding of points. Allows small-order points."],["ietf_cofactored_verify","Cofactored verification. Rejects non-canonical encoding of points. Allows small-order points."],["ietf_cofactorless_batch_verify","Batch verification. Cofactorless verification. Rejects non-canonical encoding of points. Allows small-order points. One should not use this as it can be flaky."],["ietf_cofactorless_verify","Cofactorless verification. Rejects non-canonical encoding of points. Allows small-order points."],["secret_to_public",""],["sign","Sign a message under a secret key."],["zcash_batch_verify","Batch verification. Cofactored verification. Allows non-canonical encoding of points. Allows small-order points."],["zcash_verify","Cofactored verification. Allows non-canonical encoding of points. Allows small-order points."]],"struct":[["BatchEntry",""],["BigInteger",""],["BigIntegerCanvas",""],["BigScalar",""],["BigScalarCanvas",""],["CompressedEdPoint","Fixed length byte array."],["Ed25519FieldElement",""],["FieldCanvas",""],["Scalar",""],["ScalarCanvas",""],["SerializedScalar","Fixed length byte array."],["Signature","Fixed length byte array."]],"type":[["PublicKey",""],["SecretKey",""],["VerifyResult",""]]});