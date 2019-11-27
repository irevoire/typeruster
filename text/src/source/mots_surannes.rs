pub fn get_text() -> crate::Text {
    let s = String::from(
        r#"Craint qu’il est – puisque supposément dévoreur d’enfants, de princesses et de toute chair fraîche portant jupons – le loup tient une place particulière dans le langage des années de son règne. Quand son nom fait irruption dans une expression on est dans le sérieux voire le pesant.
Sauvage et mystérieux, son évocation teinte le propos de gravité même quand il s’agit d’évoquer ses flatulences comme dans n’avoir jamais vu péter le loup sur la pierre de bois.

Car l’animal, tout redoutable qu’il soit, pète.

Tel le plus vil sconse de la forêt, le loup se laisse parfois aller au soulagement coupable d’un vent bien balancé; et l’on dira de qui l’a vu agir ainsi qu’il en connait un rayon sur la vie tant l’observation du loup ballonné est ardue.

N’avoir jamais vu péter le loup sur la pierre de bois est en effet la marque de l’ignorant des choses de l’existence, du naïf, du candide.

On notera la nécessité d‘un bois pétrifié pour que le canidé puisse se laisser aller et transmettre ainsi à qui assistera à son concert de gaz intestinaux les secrets métaphysiques les mieux gardés. Autant dire qu‘avoir vu péter le loup sur la pierre de bois est un événement exceptionnel et que les privilégiés dans la confidence sont peu nombreux.

Celui qui a vu péter le loup sur la pierre de bois ne s’en vante guère, conscient qu’il est de détenir des réponses que le commun des mortels ne peut entendre. C’est pour cela que l’expression n’existe que par la négative et que l’on se contentera de décrire l’ingénu grâce à elle.

Le loup disparaissant petit à petit des faubourgs et des forêts suite à de menus différends avec des blancs-becs prétendant tout savoir, n’avoir jamais vu péter le loup sur la pierre de bois déclinera elle aussi jusqu’à devenir totalement surannée.

Cette double extinction ouvrira une voie royale à l’homme qui a vu l’homme qui a vu l’ours, sorte de sachant moderne qui n’a pas besoin de regarder les loups se soulager pour tout savoir, lui. Mais ceci faisant appel à un autre animal, c’est évidemment une autre histoire.

"#,
    );
    crate::Text::new(s, String::from("mot surannes"))
}
