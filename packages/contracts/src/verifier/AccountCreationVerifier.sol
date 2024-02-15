// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract AccountCreationVerifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 394495987856382417489648320495229814478921837284687621672127718015891407894;
    uint256 constant deltax2 = 20003026255182470220763198008306097063266502241908640198565111261382894563584;
    uint256 constant deltay1 = 5153732139172663003461773433682950750534947429330430744721968822842995208827;
    uint256 constant deltay2 = 9602455228490272404836338596621746312766674457420929437774724091012936152901;

    
    uint256 constant IC0x = 10761170875748090112170297380238730368276891615062163268413943079435958653371;
    uint256 constant IC0y = 8944016340177153704963076504937615447109384661225789112731882124613355165601;
    
    uint256 constant IC1x = 17979990935963504933487306736336915927955798068180920219740458942815178684762;
    uint256 constant IC1y = 16651731519252621727291032465742601071725203601712801999466397627257027752439;
    
    uint256 constant IC2x = 14663083415816073773614642098554430040806805847360853621405589403936291810943;
    uint256 constant IC2y = 6317766953870455290983252471349807582261719818751587162636603167241752907019;
    
    uint256 constant IC3x = 20377132757024599402375642350276067534346274863781762490063405930966274284526;
    uint256 constant IC3y = 12086270144824945128175128808957490235743489471884156078712186793933843050515;
    
    uint256 constant IC4x = 16126469816518283218015250316726151133937714065156910136141275337000050251959;
    uint256 constant IC4y = 2882543809944777834758955897995483535988968195866892899048811406029715502558;
    
    uint256 constant IC5x = 16933676122096524832393639429006045996778001648500464809189178080785378457033;
    uint256 constant IC5y = 17675425177211431901565328496164868318795809526930475038921835665163536863805;
    
    uint256 constant IC6x = 11949822495323679488648331393913487834230781988564644883110118850042355052494;
    uint256 constant IC6y = 2114298792051364765081120188122331514146120674219559670615708764639987683578;
    
    uint256 constant IC7x = 17046242840418791255319669093641916116335302381443802342953653114019496143623;
    uint256 constant IC7y = 11226266136835877534735852382545951482471609496239681768167987376211288564459;
    
    uint256 constant IC8x = 7616070070898941974315808779493511392754098449699781176845235034505328607786;
    uint256 constant IC8y = 8973118153612121314736490009824474041817435080146517807216576036316012442875;
    
    uint256 constant IC9x = 17911893871706320472103169130449640238981055192170374352402065310737604138808;
    uint256 constant IC9y = 11791305601551499733557766166291194901125262967559914237423713749785496417898;
    
    uint256 constant IC10x = 13095809173149064990041447127638667842331098941363942128063910739947384949889;
    uint256 constant IC10y = 11623517517259510779480048136366634699145191856666061684367509912930858877561;
    
    uint256 constant IC11x = 9951106165113610997230963464215177277527154163598154321453335829293575115166;
    uint256 constant IC11y = 12048337881330301994376382472987584879401674121012526502751942835901867150834;
    
    uint256 constant IC12x = 14720279726342358412892450667847530215243501087316853231801527146749312905742;
    uint256 constant IC12y = 16371354465076218854866398746554182242515563117357477174715937450709014132326;
    
    uint256 constant IC13x = 17715568608065378010764597489935245928760845561530536927109146975431790820022;
    uint256 constant IC13y = 9654257669853909597688213072068227805115499316415452451512149706636088943268;
    
    uint256 constant IC14x = 11512969057921681794318964885528643607554619161189031136343109550170942372570;
    uint256 constant IC14y = 3020836647969348141940556963585248575639186503149333312914418617952605961660;
    
    uint256 constant IC15x = 1316180717904281335382405778620805277977806904733594414161909151779777967901;
    uint256 constant IC15y = 19299276711474140391561721956740819633578380584022009440596799512059770074508;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[15] calldata _pubSignals) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }
            
            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x
                
                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))
                
                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))
                
                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))
                
                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))
                
                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))
                
                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))
                
                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))
                
                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))
                
                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))
                
                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))
                
                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))
                
                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))
                
                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))
                
                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))
                
                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))
                

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))


                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)


                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F
            
            checkField(calldataload(add(_pubSignals, 0)))
            
            checkField(calldataload(add(_pubSignals, 32)))
            
            checkField(calldataload(add(_pubSignals, 64)))
            
            checkField(calldataload(add(_pubSignals, 96)))
            
            checkField(calldataload(add(_pubSignals, 128)))
            
            checkField(calldataload(add(_pubSignals, 160)))
            
            checkField(calldataload(add(_pubSignals, 192)))
            
            checkField(calldataload(add(_pubSignals, 224)))
            
            checkField(calldataload(add(_pubSignals, 256)))
            
            checkField(calldataload(add(_pubSignals, 288)))
            
            checkField(calldataload(add(_pubSignals, 320)))
            
            checkField(calldataload(add(_pubSignals, 352)))
            
            checkField(calldataload(add(_pubSignals, 384)))
            
            checkField(calldataload(add(_pubSignals, 416)))
            
            checkField(calldataload(add(_pubSignals, 448)))
            
            checkField(calldataload(add(_pubSignals, 480)))
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
