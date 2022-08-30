export type Shoey = {
  "version": "0.1.0",
  "name": "shoey",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "voteMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "shoeyMasterEditionMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "submit",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyEditionMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "shoeyEditionMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyEditionMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoey",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userVoteAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userShoeyEditionAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "shoeyName",
          "type": "string"
        },
        {
          "name": "editionNumber",
          "type": "u64"
        }
      ]
    },
    {
      "name": "vote",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoey",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "voterVoteAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "shoeyName",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "manager",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voteMint",
            "type": "publicKey"
          },
          {
            "name": "voteMetadata",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionMint",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionMetadata",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEdition",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionVault",
            "type": "publicKey"
          },
          {
            "name": "paymentMint",
            "type": "publicKey"
          },
          {
            "name": "paymentVault",
            "type": "publicKey"
          },
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "shoey",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "videoName",
            "type": "string"
          },
          {
            "name": "votes",
            "type": "u64"
          },
          {
            "name": "paymentVault",
            "type": "publicKey"
          },
          {
            "name": "shoeyMint",
            "type": "publicKey"
          },
          {
            "name": "manager",
            "type": "publicKey"
          },
          {
            "name": "storageAccount",
            "type": "publicKey"
          }
        ]
      }
    }
  ]
};

export const IDL: Shoey = {
  "version": "0.1.0",
  "name": "shoey",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "voteMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "shoeyMasterEditionMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "submit",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyMasterEditionVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "shoeyEditionMint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "shoeyEditionMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyEdition",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoeyEditionMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoey",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "userVoteAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userShoeyEditionAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "shoeyName",
          "type": "string"
        },
        {
          "name": "editionNumber",
          "type": "u64"
        }
      ]
    },
    {
      "name": "vote",
      "accounts": [
        {
          "name": "voteMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "manager",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "paymentMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "paymentVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "shoey",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "voter",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "voterVoteAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "shoeyName",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "manager",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voteMint",
            "type": "publicKey"
          },
          {
            "name": "voteMetadata",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionMint",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionMetadata",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEdition",
            "type": "publicKey"
          },
          {
            "name": "shoeyMasterEditionVault",
            "type": "publicKey"
          },
          {
            "name": "paymentMint",
            "type": "publicKey"
          },
          {
            "name": "paymentVault",
            "type": "publicKey"
          },
          {
            "name": "admin",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "shoey",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "videoName",
            "type": "string"
          },
          {
            "name": "votes",
            "type": "u64"
          },
          {
            "name": "paymentVault",
            "type": "publicKey"
          },
          {
            "name": "shoeyMint",
            "type": "publicKey"
          },
          {
            "name": "manager",
            "type": "publicKey"
          },
          {
            "name": "storageAccount",
            "type": "publicKey"
          }
        ]
      }
    }
  ]
};
