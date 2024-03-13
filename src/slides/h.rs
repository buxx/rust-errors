#![allow(warnings, unused)]
// 🐜 GESTION D'ERREURS 🐛
//       Cas métier
//
// Cas d'erreurs :
//
//   1/ Ajout de tâches
//      - Mots-clefs interdit (ex. "learn php")
//        --> `UserInputError`
//        --> "🤔 User input error: 🚫 Forbidden value: '<détails>'"
//
//   2/ Coche de tâches
//      - Tâche non trouvé
//        --> `UserInputError`
//        --> "🤔 User input error: ❓ Unknown task '<détails>'"
//
//   Autres :
//     - Erreurs I/O
//        --> `InternalError`
//       --> "💥 Internal error: 💾 Disk error: '<détails>'"
