use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;

/// Generic function to convert non-signer data from the operator state retriever format
/// to a generic BLS signature checker format using the blssigcheckoperatorstateretriever bindings.
/// This is generic BLS functionality that belongs in the BLS/EigenLayer executor.
pub fn convert_non_signer_data(
    non_signer_data: getNonSignerStakesAndSignatureReturn,
) -> crate::bindings::blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature{
    crate::bindings::blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
        nonSignerQuorumBitmapIndices: non_signer_data._0.nonSignerQuorumBitmapIndices,
        nonSignerPubkeys: non_signer_data
            ._0
            .nonSignerPubkeys
            .into_iter()
            .map(|p| crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point { X: p.X, Y: p.Y })
            .collect(),
        quorumApks: non_signer_data
            ._0
            .quorumApks
            .into_iter()
            .map(|p| crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point { X: p.X, Y: p.Y })
            .collect(),
        apkG2: crate::bindings::blssigcheckoperatorstateretriever::BN254::G2Point {
            X: non_signer_data._0.apkG2.X,
            Y: non_signer_data._0.apkG2.Y,
        },
        sigma: crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
            X: non_signer_data._0.sigma.X,
            Y: non_signer_data._0.sigma.Y,
        },
        quorumApkIndices: non_signer_data._0.quorumApkIndices,
        totalStakeIndices: non_signer_data._0.totalStakeIndices,
        nonSignerStakeIndices: non_signer_data._0.nonSignerStakeIndices,
    }
}
