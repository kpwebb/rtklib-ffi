#![allow(warnings)] 

use std::option::Iter;
use std::{fs::File, io::Read, mem::MaybeUninit, ptr::addr_of_mut};

use rtcm_rs::{Message, MsgFrameIter};
use rtklib_sys::rtklib::{self, decode_msm7, obsd_t, rtcm_t};




#[test]
fn process_rtcm_parsing() {
    // let ref mut cool = CoolStruct {x: 0, y: 0};
    let file_path = "tests/debug.rtcm";
    // unsafe { cool_function(2, 6, cool) }
    let mut rtcm_file = File::open(file_path).expect(format!("Unable to open file: {}", file_path).as_str());

    let mut rtcm_buffer = Vec::<u8>::new();

    if let Ok(_) = rtcm_file.read_to_end(&mut rtcm_buffer) {

        let mut iterator = MsgFrameIter::new(rtcm_buffer.as_slice());

        let mut test_1077 = false;
        let mut test_1097 = false;

        for message_frame in &mut iterator {
            match message_frame.get_message() {

                Message::Msg1077(msg1077) => {
                    if test_1077 == false {

                        unsafe { 
                            let mut rtcm:MaybeUninit<rtcm_t> = MaybeUninit::zeroed();
                            let rtcm_ptr = rtcm.as_mut_ptr();
    
                            
                            let mut buff:[u8;1200] = [0;1200]; 
                            let mut i = 0;
                            for b in message_frame.frame_data() {
                                buff[i] = *b;
                                i += 1;
                            }
    
                            let mut rtklib_observations:MaybeUninit<[obsd_t;24]>= MaybeUninit::zeroed(); 
                            let rtklib_observations_ptr = rtklib_observations.assume_init_mut().as_mut_ptr();
    
                            addr_of_mut!((*rtcm_ptr).obs.data).write(rtklib_observations_ptr);
                            addr_of_mut!((*rtcm_ptr).buff).write(buff);
                            addr_of_mut!((*rtcm_ptr).len).write( message_frame.frame_len() as i32);
                            
                            // calc rtklib values
                            decode_msm7(rtcm.as_mut_ptr(), 0x01);
    
                            // calc rtcmlib values
                            //let rtcmlib_observations = process_msm1077(msg1077);
    
                            for rtklib_obs in rtklib_observations.assume_init() {
    
                                println!("{}", rtklib_obs.D[0]);
    
                                assert_eq!(rtklib_obs.sat, 10);
                                test_1077 = true;
                                break;;
                            }
                        }
                    }
                }

                // galileo  
                Message::Msg1097(msg1097) => {
                    
                    println!("{}", message_frame.message_number().unwrap());
                    if test_1097 == false {
                        unsafe { 
                            let mut rtcm:MaybeUninit<rtcm_t> = MaybeUninit::zeroed();
                            let rtcm_ptr = rtcm.as_mut_ptr();
    
                            
                            let mut buff:[u8;1200] = [0;1200]; 
                            let mut i = 0;
                            for b in message_frame.frame_data() {
                                buff[i] = *b;
                                i += 1;
                            }
    
                            let mut rtklib_observations:MaybeUninit<[obsd_t;24]>= MaybeUninit::zeroed(); 
                            let rtklib_observations_ptr = rtklib_observations.assume_init_mut().as_mut_ptr();
    
                            addr_of_mut!((*rtcm_ptr).obs.data).write(rtklib_observations_ptr);
                            addr_of_mut!((*rtcm_ptr).buff).write(buff);
                            addr_of_mut!((*rtcm_ptr).len).write( message_frame.frame_len() as i32);
                            
                            // calc rtklib values
                            decode_msm7(rtcm.as_mut_ptr(), 0x08);
    
                            // calc rtcmlib values
                            //let rtcmlib_observations = process_msm1097(msg1097);
    
                            for rtklib_obs in rtklib_observations.assume_init() {
    
                                println!("{}", rtklib_obs.D[0]);
                                assert_eq!(rtklib_obs.sat, 36);
                                test_1097 = true;;
                                break;
                            }
                        }
                    }
                }

                _ => {
                    println!("{}", message_frame.message_number().unwrap());
                }
            }
            
        }

        assert!(test_1077);
        assert!(test_1097);
    }
    
    
}

