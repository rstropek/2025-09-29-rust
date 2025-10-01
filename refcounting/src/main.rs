#![allow(dead_code, unused_variables)]

use std::{cell::RefCell, rc::Rc, sync::RwLock};

struct Ring {
    engraving: String,
}

fn main() {
    {
        let saurons_ring = Ring {
            engraving: String::from(
                "One Ring to rule them all, One Ring to find them, One Ring to bring them all and in the darkness bind them.",
            ),
        };
        
        let saurons_ring = Rc::new(saurons_ring);
        // saurons_ring.engraving.push_str(" Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.");
        println!("Address of RC with saurons_ring: {:p}", &saurons_ring);
        println!("Address of saurons_ring: {:p}", saurons_ring.as_ref());
        println!(
            "The ring has now {} owners",
            Rc::strong_count(&saurons_ring)
        );

        // Something strange happens here
        let frodos_ring = saurons_ring.clone();
        println!("Address of RC with frodos_ring: {:p}", &frodos_ring);
        println!("Address of frodos_ring: {:p}", frodos_ring.as_ref());
        println!(
            "The ring has now {} owners",
            Rc::strong_count(&saurons_ring)
        );

        println!("Frodos ring engraving: {}", frodos_ring.engraving);
        drop(frodos_ring);
        println!(
            "The ring has now {} owners",
            Rc::strong_count(&saurons_ring)
        );

        println!("Saurons ring engraving: {}", saurons_ring.engraving);
    }

    {
        let ring = Rc::new(RwLock::new(Ring {
            engraving: String::from(""),
        }));

        {
            let mut ring_write = ring.write().unwrap();
            ring_write.engraving.push_str("Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.");
        }

        {
            let ring_read = ring.read().unwrap();
            let ring_read_2 = ring.read().unwrap();
            println!("Ring engraving: {}", ring_read.engraving);
            println!("Ring engraving: {}", ring_read_2.engraving);
        }
    }

    {
        let ring = Rc::new(RefCell::new(Ring {
            engraving: String::from(""),
        }));

        {
            let mut ring_borrow_mut = ring.borrow_mut();
            //let mut ring_borrow_mut_2 = ring.borrow_mut();
            ring_borrow_mut.engraving.push_str("Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.");
            //ring_borrow_mut_2.engraving.push_str("Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.");
        }

        println!("Content of engraving: {}", ring.borrow().engraving);
    }
}
