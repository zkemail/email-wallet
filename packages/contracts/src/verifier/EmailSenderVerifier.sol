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

contract EmailSenderVerifier {
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
    uint256 constant deltax1 = 10857059420391799529676027676111621705215160710843885983002383074666004320175;
    uint256 constant deltax2 = 1492213702940301485870937895425416400577229330428083750548471315770930363407;
    uint256 constant deltay1 = 695612864408011321833674252899170776927620785441444797974898965699571483306;
    uint256 constant deltay2 = 12958759356356810206657207918502648331110502686774147868047751178414163345650;

    
    uint256 constant IC0x = 3661811636740465097471627748494142914168777603911298107803372416407751499801;
    uint256 constant IC0y = 17537560584991684631031314745946481428450739569701718381851965259503495463806;
    
    uint256 constant IC1x = 6519583210212790063106035096353874925192735060199973029029945601940390581655;
    uint256 constant IC1y = 9630325481838952655216314216814137249556049575456765541321359000203192702694;
    
    uint256 constant IC2x = 5112054690601779146371895981370824612561203753111666857923896289388171176731;
    uint256 constant IC2y = 5496723370749934855673779696174770531612814369778526904953353054156175844699;
    
    uint256 constant IC3x = 13976205454172895292782845045786530378964820071407584961717692196579009154525;
    uint256 constant IC3y = 21170309788421777508555903983144239031359701215385954557160226064714790728728;
    
    uint256 constant IC4x = 18325540875860213863790638025497283183035765241476372375313764506683272843532;
    uint256 constant IC4y = 10853448229002619309692045659166205514605026915260753531008448201024165665302;
    
    uint256 constant IC5x = 5615874314377852163682977933818214241179077817275906980019279902208787384232;
    uint256 constant IC5y = 9132443273319153745924314740486249989432507741497664120596747166679196060242;
    
    uint256 constant IC6x = 12244314866363248792372803797189895495958764232059846053347394459809648330507;
    uint256 constant IC6y = 16675131219260940080447083475222096036178522885874801648116421125598946784931;
    
    uint256 constant IC7x = 6212247510807206865294815903761611416561242644656555232709605429168014856594;
    uint256 constant IC7y = 3758187135890079457226390663238655889853796516488517296860537836813682671216;
    
    uint256 constant IC8x = 15815454263907700126694625683418015132967516135368764362274936596680832985562;
    uint256 constant IC8y = 21876473152892411267184630997504122830832569800832955072382061208882720656495;
    
    uint256 constant IC9x = 10986464527209282313702025395569357184144660224825765482923286521986096458237;
    uint256 constant IC9y = 13337432793251995398673403167874434892824211217513919966763653546049396531453;
    
    uint256 constant IC10x = 3398814180813447525221765885446903088169760458568327329995619175600121462982;
    uint256 constant IC10y = 4535184538598630837309124912928589220254832086343862090930615742634287985681;
    
    uint256 constant IC11x = 2704768086814132264357683490371968593804510642680067040563037904508230325263;
    uint256 constant IC11y = 1972913450067184033830350750273096888612652694116354266196558424780096995068;
    
    uint256 constant IC12x = 3776432724543685682193838158334601165368695031388298868777172055778328003018;
    uint256 constant IC12y = 11829164533765051770697797843397441436619085979074995556313107083369269600439;
    
    uint256 constant IC13x = 15783614549223406163298229722725723244538974212652276644325206794043646601580;
    uint256 constant IC13y = 16043211739243495720169845541807880280926672573123673330236702225527499370405;
    
    uint256 constant IC14x = 7889485879715098249265375571789641734778838616834941261333878472080531247138;
    uint256 constant IC14y = 4776574879521453870180077076566767922247378080116033855423844067807932523934;
    
    uint256 constant IC15x = 963548392121935355434194259456419883927996181895112301468476955427104787909;
    uint256 constant IC15y = 10653161239029199911468988487797516858779108339481921370779320530728778363412;
    
    uint256 constant IC16x = 11432576194333430644131665050299881998494259867377180345063559088774092434698;
    uint256 constant IC16y = 21134020885809678749982342514745531178311282743390232149556965986315987527045;
    
    uint256 constant IC17x = 20006874229857420035987492838801041526604354652289530778131848647775761951161;
    uint256 constant IC17y = 9865340621990586941478969730874918109609549579422620026040328447426575107889;
    
    uint256 constant IC18x = 11623455766790371300253909391097209939297636070766571645609957773166787493036;
    uint256 constant IC18y = 11194826308852612754030389602248210975136721673236763860866094144479910168377;
    
    uint256 constant IC19x = 16028947117975856167783261967683451252499535009485804376628142342265871280121;
    uint256 constant IC19y = 21368715425499421632124500287201110451269665003294717222380222397405464516028;
    
    uint256 constant IC20x = 11698793155137238295562358880379369989954869659593829307064854287027073965939;
    uint256 constant IC20y = 3212057740937125841734146336625984252258244842314313164395679232195612597815;
    
    uint256 constant IC21x = 18015358290082528516632587899939817133844283111003476462055084254622743287832;
    uint256 constant IC21y = 17951927596341939794996636551763948575045667492292965971030207960504663729215;
    
    uint256 constant IC22x = 18781359464943171216676789802595065570346237704366618351077209741040764017972;
    uint256 constant IC22y = 8700237979909647197526038330546424358811496431951502517050255566836303666545;
    
    uint256 constant IC23x = 1913150227415318955351820273265255879997235826402682932674057596946198234929;
    uint256 constant IC23y = 8885953376051308335208102732619010275072456954802599410846516293934634587374;
    
    uint256 constant IC24x = 19554707501571067446796910219013557851254599704585317846600543354011878491136;
    uint256 constant IC24y = 13372595098746446047366236616399997066349433078399789341807169508599863452112;
    
    uint256 constant IC25x = 5243283402563402226709457761321879595565064130029746885147890298863301958791;
    uint256 constant IC25y = 14290862098015848979981054674749884351769392856004702669715693571099191248680;
    
    uint256 constant IC26x = 12927751040345873196815872355881883311082035987342736287596231630173333993550;
    uint256 constant IC26y = 13061535120723252029689333503554317177137306442566310245224697463445422607005;
    
    uint256 constant IC27x = 15555563296394519415751545049235498073107463937454351451649459438450978535522;
    uint256 constant IC27y = 15917700347720269364006297900597258821465392549733842632870003900032334334641;
    
    uint256 constant IC28x = 4298983234019286082687176793915926591648257293646055425268357091675309611572;
    uint256 constant IC28y = 9102209347374058701675073743530668453166696038920003835945771962053546406785;
    
    uint256 constant IC29x = 21102197781335063304050793876989258308673800374586843758381516573986974202485;
    uint256 constant IC29y = 13987074081104915833362408669098425302779526325994354269791704174888757969239;
    
    uint256 constant IC30x = 21377682269066401704785912665945228303115514287093044879401187617891490114690;
    uint256 constant IC30y = 7042668069545413277953149106287686606997361129876492263442774428343687489247;
    
    uint256 constant IC31x = 4601487642931269207527448978274328397803377966780810182958073285150511895154;
    uint256 constant IC31y = 6278626838008658182653032654155965133145995925815950311617560638010046031567;
    
    uint256 constant IC32x = 16655361888715101062686947419344920395651882710937619333597462357599795632477;
    uint256 constant IC32y = 2970744159574829429884150440775198910181566150556734678193036680859399894057;
    
    uint256 constant IC33x = 3720146037000511806783892469803950379831459774192294715405352472608977881776;
    uint256 constant IC33y = 1659044873001096071942440974001800934122508782028053233471865455964526455406;
    
    uint256 constant IC34x = 8003785004906728459568672578163818518370253956983599169625398987555930754615;
    uint256 constant IC34y = 7156624929831114274607920807088678020799024943902246023662022955511134413378;
    
    uint256 constant IC35x = 19618027247315650370632705974342575103020620847421596320369416001679436229567;
    uint256 constant IC35y = 9957570949335616844020321144879920401256407424736470236168501567674581894403;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[35] calldata _pubSignals) public view returns (bool) {
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
                
                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))
                
                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))
                
                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))
                
                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))
                
                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))
                
                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))
                
                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))
                
                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))
                
                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))
                
                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))
                
                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))
                
                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))
                
                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))
                
                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))
                
                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))
                
                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))
                
                g1_mulAccC(_pVk, IC32x, IC32y, calldataload(add(pubSignals, 992)))
                
                g1_mulAccC(_pVk, IC33x, IC33y, calldataload(add(pubSignals, 1024)))
                
                g1_mulAccC(_pVk, IC34x, IC34y, calldataload(add(pubSignals, 1056)))
                
                g1_mulAccC(_pVk, IC35x, IC35y, calldataload(add(pubSignals, 1088)))
                

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
            
            checkField(calldataload(add(_pubSignals, 512)))
            
            checkField(calldataload(add(_pubSignals, 544)))
            
            checkField(calldataload(add(_pubSignals, 576)))
            
            checkField(calldataload(add(_pubSignals, 608)))
            
            checkField(calldataload(add(_pubSignals, 640)))
            
            checkField(calldataload(add(_pubSignals, 672)))
            
            checkField(calldataload(add(_pubSignals, 704)))
            
            checkField(calldataload(add(_pubSignals, 736)))
            
            checkField(calldataload(add(_pubSignals, 768)))
            
            checkField(calldataload(add(_pubSignals, 800)))
            
            checkField(calldataload(add(_pubSignals, 832)))
            
            checkField(calldataload(add(_pubSignals, 864)))
            
            checkField(calldataload(add(_pubSignals, 896)))
            
            checkField(calldataload(add(_pubSignals, 928)))
            
            checkField(calldataload(add(_pubSignals, 960)))
            
            checkField(calldataload(add(_pubSignals, 992)))
            
            checkField(calldataload(add(_pubSignals, 1024)))
            
            checkField(calldataload(add(_pubSignals, 1056)))
            
            checkField(calldataload(add(_pubSignals, 1088)))
            
            checkField(calldataload(add(_pubSignals, 1120)))
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
