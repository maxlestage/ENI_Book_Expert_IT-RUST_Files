// ici on est dans le module racine implicite du crate.

fn main() {
    println!("Exemple crate France");
}

mod auvergne_rhone_alpes {

    // On définit des choses relatives à Auvergne-Rhône-Alpes

    mod ain {

        // On définit des choses relatives à l'Ain.
    }

    mod isere {

        // On définit des choses relatives à l'Isère.
     
    }
}

mod provence_alpes_cotes_d_azur {

    // On définit des choses relatives à Provence-Alpes-Côte-d'Azur.

    mod bouches_du_rhone {

        // On définit des choses relatives aux Bouches-du-Rhône.
    }

    mod hautes_alpes {

        // On définit des choses relatives aux Hautes-Alpes.
     
    }

    mod vaucluse {

        // On définit des choses relatives au Vaucluse.
     
    }
}
