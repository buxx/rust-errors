#![allow(warnings, unused)]
// 🐜 GESTION D'ERREURS 🐛
//       Cas métier
//
// Cas d'erreurs :
//
//   1/ Ajout de tâches
//      - Mots-clefs interdit (ex. "learn php")
//        --> "🤔 User input error: 🚫 Forbidden value: '<détails>'"
//
//   2/ Coche de tâches
//      - Tâche non trouvé
//        --> "🤔 User input error: ❓ Unknown task '<détails>'"
//
//   Autres :
//     - Erreurs I/O
//       --> "💥 Internal error: 💾 Disk error: '<détails>'"
