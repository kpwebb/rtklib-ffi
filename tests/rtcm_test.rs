use std::{fs::File, io::Read, mem::MaybeUninit, ptr::addr_of_mut};
use rtcm_rs::MsgFrameIter;
use rtklib_sys::{decode_msm7, obsd_t, rtcm_t};

#[test]
fn parse_rtcm_1077() {
    // let ref mut cool = CoolStruct {x: 0, y: 0};
    let file_path = "tests/debug.rtcm";
    // unsafe { cool_function(2, 6, cool) }
    let mut rtcm_file = File::open(file_path).expect(format!("Unable to open file: {}", file_path).as_str());

    let mut rtcm_buffer = Vec::<u8>::new();

    if let Ok(_) = rtcm_file.read_to_end(&mut rtcm_buffer) {

        let mut iterator = MsgFrameIter::new(rtcm_buffer.as_slice());

        let mut first_msg1077 = true;
        for message_frame in &mut iterator {
            if message_frame.message_number().unwrap() == 1077 {
                unsafe { 
                    let mut rtcm:MaybeUninit<rtcm_t> = MaybeUninit::zeroed();
                    let rtcm_ptr = rtcm.as_mut_ptr();

                    
                    let mut buff:[u8;1200] = [0;1200]; 
                    let mut i = 0;
                    for b in message_frame.frame_data() {
                        buff[i] = *b;
                        i += 1;
                    }

                    let mut obs_data:MaybeUninit<[obsd_t;24]>= MaybeUninit::zeroed(); 
                    let obs_data_ptr = obs_data.assume_init_mut().as_mut_ptr();

                    addr_of_mut!((*rtcm_ptr).obs.data).write(obs_data_ptr);
                    addr_of_mut!((*rtcm_ptr).buff).write(buff);
                    addr_of_mut!((*rtcm_ptr).len).write( message_frame.frame_len() as i32);
                    
                    decode_msm7(rtcm.as_mut_ptr(), 0x01);

                    let mut obs_stats = 0;
                    for obs in obs_data.assume_init().iter() {
                        if obs.sat > 0 {
                            obs_stats += 1;
                        }
                    }    

                    if first_msg1077 {
                        assert_eq!(obs_stats, 8, "testing number of observed signals {} equals {}", obs_stats, 8);
                        first_msg1077 = false;
                    }
                    else {
                        assert_eq!(obs_stats, 7, "testing number of observed signals {} equals {}", obs_stats, 7);
                        return;
                    }
                    
                };
            }
        }
    }
}

