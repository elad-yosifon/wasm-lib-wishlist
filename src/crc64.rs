use wasm_bindgen::prelude::*;

/*
// generate lookup table using 0xAD93D23594C935A9 polynomial (reverse order)
fn fill_crc_table(poly: u64) -> [u64; 256] {
   let mut lut: [u64; 256] = [0; 256];
   let mut crc: u64;
   for i in 0..256 {
       crc = i;
       for _j in 0..8 {
           if crc & 1 == 1 {
               crc >>= 1;
               crc ^= poly;
           } else {
               crc >>= 1;
           }
       }
       lut[i as usize] = crc;
   }
   lut
}

*/
const CRC64_LOOKUP_TABLE: [u64; 256] = [
    //@formatter:off
    0x0000000000000000, 0xDC6604FC730F8E5D, 0xE3EBAD93CF8D77E9, 0x3F8DA96FBC82F9B4,
    0x9CF0FF4CB6888481, 0x4096FBB0C5870ADC, 0x7F1B52DF7905F368, 0xA37D56230A0A7D35,
    0x62C65AF244836251, 0xBEA05E0E378CEC0C, 0x812DF7618B0E15B8, 0x5D4BF39DF8019BE5,
    0xFE36A5BEF20BE6D0, 0x2250A1428104688D, 0x1DDD082D3D869139, 0xC1BB0CD14E891F64,
    0xC58CB5E48906C4A2, 0x19EAB118FA094AFF, 0x26671877468BB34B, 0xFA011C8B35843D16,
    0x597C4AA83F8E4023, 0x851A4E544C81CE7E, 0xBA97E73BF00337CA, 0x66F1E3C7830CB997,
    0xA74AEF16CD85A6F3, 0x7B2CEBEABE8A28AE, 0x44A142850208D11A, 0x98C7467971075F47,
    0x3BBA105A7B0D2272, 0xE7DC14A60802AC2F, 0xD851BDC9B480559B, 0x437B935C78FDBC6 ,
    0xD03ECFA23B9FE217, 0xC58CB5E48906C4A , 0x33D56231F41295FE, 0xEFB366CD871D1BA3,
    0x4CCE30EE8D176696, 0x90A83412FE18E8CB, 0xAF259D7D429A117F, 0x7343998131959F22,
    0xB2F895507F1C8046, 0x6E9E91AC0C130E1B, 0x511338C3B091F7AF, 0x8D753C3FC39E79F2,
    0x2E086A1CC99404C7, 0xF26E6EE0BA9B8A9A, 0xCDE3C78F0619732E, 0x1185C3737516FD73,
    0x15B27A46B29926B5, 0xC9D47EBAC196A8E8, 0xF659D7D57D14515C, 0x2A3FD3290E1BDF01,
    0x8942850A0411A234, 0x552481F6771E2C69, 0x6AA92899CB9CD5DD, 0xB6CF2C65B8935B80,
    0x777420B4F61A44E4, 0xAB1224488515CAB9, 0x949F8D273997330D, 0x48F989DB4A98BD50,
    0xEB84DFF84092C065, 0x37E2DB04339D4E38, 0x86F726B8F1FB78C , 0xD4097697FC1039D1,
    0xFB5A3B2F5EADAF7D, 0x273C3FD32DA22120, 0x18B196BC9120D894, 0xC4D79240E22F56C9,
    0x67AAC463E8252BFC, 0xBBCCC09F9B2AA5A1, 0x844169F027A85C15, 0x58276D0C54A7D248,
    0x999C61DD1A2ECD2C, 0x45FA652169214371, 0x7A77CC4ED5A3BAC5, 0xA611C8B2A6AC3498,
    0x56C9E91ACA649AD , 0xD90A9A6DDFA9C7F0, 0xE6873302632B3E44, 0x3AE137FE1024B019,
    0x3ED68ECBD7AB6BDF, 0xE2B08A37A4A4E582, 0xDD3D235818261C36, 0x15B27A46B29926B ,
    0xA22671876123EF5E, 0x7E40757B122C6103, 0x41CDDC14AEAE98B7, 0x9DABD8E8DDA116EA,
    0x5C10D4399328098E, 0x8076D0C5E02787D3, 0xBFFB79AA5CA57E67, 0x639D7D562FAAF03A,
    0xC0E02B7525A08D0F, 0x1C862F8956AF0352, 0x230B86E6EA2DFAE6, 0xFF6D821A992274BB,
    0x2B64F48D65324D6A, 0xF702F071163DC337, 0xC88F591EAABF3A83, 0x14E95DE2D9B0B4DE,
    0xB7940BC1D3BAC9EB, 0x6BF20F3DA0B547B6, 0x547FA6521C37BE02, 0x8819A2AE6F38305F,
    0x49A2AE7F21B12F3B, 0x95C4AA8352BEA166, 0xAA4903ECEE3C58D2, 0x762F07109D33D68F,
    0xD55251339739ABBA, 0x93455CFE43625E7 , 0x36B9FCA058B4DC53, 0xEADFF85C2BBB520E,
    0xEEE84169EC3489C8, 0x328E45959F3B0795, 0xD03ECFA23B9FE21 , 0xD165E80650B6707C,
    0x7218BE255ABC0D49, 0xAE7EBAD929B38314, 0x91F313B695317AA0, 0x4D95174AE63EF4FD,
    0x8C2E1B9BA8B7EB99, 0x50481F67DBB865C4, 0x6FC5B608673A9C70, 0xB3A3B2F41435122D,
    0x10DEE4D71E3F6F18, 0xCCB8E02B6D30E145, 0xF3354944D1B218F1, 0x2F534DB8A2BD96AC,
    0xAD93D23594C935A9, 0x71F5D6C9E7C6BBF4, 0x4E787FA65B444240, 0x921E7B5A284BCC1D,
    0x31632D792241B128, 0xED052985514E3F75, 0xD28880EAEDCCC6C1, 0xEEE84169EC3489C ,
    0xCF5588C7D04A57F8, 0x13338C3BA345D9A5, 0x2CBE25541FC72011, 0xF0D821A86CC8AE4C,
    0x53A5778B66C2D379, 0x8FC3737715CD5D24, 0xB04EDA18A94FA490, 0x6C28DEE4DA402ACD,
    0x681F67D11DCFF10B, 0xB479632D6EC07F56, 0x8BF4CA42D24286E2, 0x5792CEBEA14D08BF,
    0xF4EF989DAB47758A, 0x28899C61D848FBD7, 0x1704350E64CA0263, 0xCB6231F217C58C3E,
    0xAD93D23594C935A , 0xD6BF39DF2A431D07, 0xE93290B096C1E4B3, 0x3554944CE5CE6AEE,
    0x9629C26FEFC417DB, 0x4A4FC6939CCB9986, 0x75C26FFC20496032, 0xA9A46B005346EE6F,
    0x7DAD1D97AF56D7BE, 0xA1CB196BDC5959E3, 0x9E46B00460DBA057, 0x4220B4F813D42E0A,
    0xE15DE2DB19DE533F, 0x3D3BE6276AD1DD62, 0x2B64F48D65324D6 , 0xDED04BB4A55CAA8B,
    0x1F6B4765EBD5B5EF, 0xC30D439998DA3BB2, 0xFC80EAF62458C206, 0x20E6EE0A57574C5B,
    0x839BB8295D5D316E, 0x5FFDBCD52E52BF33, 0x607015BA92D04687, 0xBC161146E1DFC8DA,
    0xB821A8732650131C, 0x6447AC8F555F9D41, 0x5BCA05E0E9DD64F5, 0x87AC011C9AD2EAA8,
    0x24D1573F90D8979D, 0xF8B753C3E3D719C0, 0xC73AFAAC5F55E074, 0x1B5CFE502C5A6E29,
    0xDAE7F28162D3714D, 0x681F67D11DCFF10 , 0x390C5F12AD5E06A4, 0xE56A5BEEDE5188F9,
    0x46170DCDD45BF5CC, 0x9A710931A7547B91, 0xA5FCA05E1BD68225, 0x799AA4A268D90C78,
    0x56C9E91ACA649AD4, 0x8AAFEDE6B96B1489, 0xB522448905E9ED3D, 0x6944407576E66360,
    0xCA3916567CEC1E55, 0x165F12AA0FE39008, 0x29D2BBC5B36169BC, 0xF5B4BF39C06EE7E1,
    0x340FB3E88EE7F885, 0xE869B714FDE876D8, 0xD7E41E7B416A8F6C, 0xB821A8732650131 ,
    0xA8FF4CA4386F7C04, 0x749948584B60F259, 0x4B14E137F7E20BED, 0x9772E5CB84ED85B0,
    0x93455CFE43625E76, 0x4F235802306DD02B, 0x70AEF16D8CEF299F, 0xACC8F591FFE0A7C2,
    0xFB5A3B2F5EADAF7 , 0xD3D3A74E86E554AA, 0xEC5E0E213A67AD1E, 0x30380ADD49682343,
    0xF183060C07E13C27, 0x2DE502F074EEB27A, 0x1268AB9FC86C4BCE, 0xCE0EAF63BB63C593,
    0x6D73F940B169B8A6, 0xB115FDBCC26636FB, 0x8E9854D37EE4CF4F, 0x52FE502F0DEB4112,
    0x86F726B8F1FB78C3, 0x5A91224482F4F69E, 0x651C8B2B3E760F2A, 0xB97A8FD74D798177,
    0x1A07D9F44773FC42, 0xC661DD08347C721F, 0xF9EC746788FE8BAB, 0x258A709BFBF105F6,
    0xE4317C4AB5781A92, 0x385778B6C67794CF, 0x7DAD1D97AF56D7B , 0xDBBCD52509FAE326,
    0x78C1830603F09E13, 0xA4A787FA70FF104E, 0x9B2A2E95CC7DE9FA, 0x474C2A69BF7267A7,
    0x437B935C78FDBC61, 0x9F1D97A00BF2323C, 0xA0903ECFB770CB88, 0x7CF63A33C47F45D5,
    0xDF8B6C10CE7538E0, 0x3ED68ECBD7AB6BD , 0x3C60C18301F84F09, 0xE006C57F72F7C154,
    0x21BDC9AE3C7EDE30, 0xFDDBCD524F71506D, 0xC256643DF3F3A9D9, 0x1E3060C180FC2784,
    0xBD4D36E28AF65AB1, 0x612B321EF9F9D4EC, 0x5EA69B71457B2D58, 0x82C09F8D3674A305,
    //@formatter:on
];

fn crc64_runner(lut: [u64; 256], str: &str) -> u64 {
    let mut crc: u64 = 0xFFFFFFFFFFFFFFFF;
    let bytes = str.as_bytes();
    for &byte in bytes.iter() {
        let index = (((byte as u64) ^ crc) & 0xFF) as usize;
        crc = lut[index] ^ ((crc >> 8) & 0xFFFFFF);
    }
    crc = !crc;
    crc
}

fn crc64(s: &str) -> u64 {
    crc64_runner(CRC64_LOOKUP_TABLE, s)
}

#[wasm_bindgen]
pub fn crc64_be(s: &str) -> Box<[u8]> {
    let crc = crc64(s);
    let vec = vec![
        ((crc >> 56) & 0xFF) as u8,
        ((crc >> 48) & 0xFF) as u8,
        ((crc >> 40) & 0xFF) as u8,
        ((crc >> 32) & 0xFF) as u8,
        ((crc >> 24) & 0xFF) as u8,
        ((crc >> 16) & 0xFF) as u8,
        ((crc >> 8) & 0xFF) as u8,
        (crc & 0xFF) as u8
    ];
    return vec.into_boxed_slice();
}

#[wasm_bindgen]
pub fn crc64_le(s: &str) -> Box<[u8]> {
    let crc = crc64(s);
    let vec = vec![
        (crc & 0xFF) as u8,
        ((crc >> 8) & 0xFF) as u8,
        ((crc >> 16) & 0xFF) as u8,
        ((crc >> 24) & 0xFF) as u8,
        ((crc >> 32) & 0xFF) as u8,
        ((crc >> 40) & 0xFF) as u8,
        ((crc >> 48) & 0xFF) as u8,
        ((crc >> 56) & 0xFF) as u8
    ];
    return vec.into_boxed_slice();
}