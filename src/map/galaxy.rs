use crate::map::sector::Sector;

pub struct Galaxy;

impl Galaxy {
    /// Create the default strategic map with chokepoints and multiple paths
    pub fn create_default_map() -> Vec<Sector> {
        vec![
            // Core Systems (0-4)
            Sector::new(0, "Sol System".to_string(), vec![1, 2, 3]),         // Earth - central hub
            Sector::new(1, "Alpha Centauri".to_string(), vec![0, 4, 5]),     // Major junction
            Sector::new(2, "Sirius".to_string(), vec![0, 6, 7]),             // Another junction  
            Sector::new(3, "Vega".to_string(), vec![0, 8]),                  // Defensive position
            
            // Mid-Ring Systems (4-8)
            Sector::new(4, "Proxima Station".to_string(), vec![1, 9]),       // Chokepoint
            Sector::new(5, "Wolf 359".to_string(), vec![1, 6, 10]),          // Crossroads
            Sector::new(6, "Tau Ceti".to_string(), vec![2, 5, 11]),          // Strategic center
            Sector::new(7, "Epsilon Eridani".to_string(), vec![2, 12]),      // Resource rich
            Sector::new(8, "Altair".to_string(), vec![3, 13]),               // Isolated outpost
            
            // Outer Systems (9-13)
            Sector::new(9, "Barnard's Star".to_string(), vec![4, 14]),       // Frontier
            Sector::new(10, "Ross 128".to_string(), vec![5, 11, 14]),        // Trade hub
            Sector::new(11, "Lacaille".to_string(), vec![6, 10, 15]),        // Industrial
            Sector::new(12, "Gliese 667C".to_string(), vec![7, 15]),         // Mining colony
            Sector::new(13, "Kepler Station".to_string(), vec![8, 16]),      // Research post
            
            // Edge Systems (14-16)
            Sector::new(14, "Asteroid Belt Omega".to_string(), vec![9, 10]), // Resource cache
            Sector::new(15, "Nebula Outpost".to_string(), vec![11, 12]),     // Hidden base
            Sector::new(16, "Deep Space Relay".to_string(), vec![13]),       // Remote station
        ]
    }
    
    /// Create a smaller tactical map for quick games
    pub fn create_tactical_map() -> Vec<Sector> {
        vec![
            Sector::new(0, "Command Base Alpha".to_string(), vec![1, 2]),
            Sector::new(1, "Orbital Platform".to_string(), vec![0, 2, 3]),
            Sector::new(2, "Asteroid Field".to_string(), vec![0, 1, 4]),
            Sector::new(3, "Supply Depot".to_string(), vec![1, 4, 5]),
            Sector::new(4, "Central Nexus".to_string(), vec![2, 3, 5, 6]),
            Sector::new(5, "Mining Colony".to_string(), vec![3, 4, 7]),
            Sector::new(6, "Defense Grid".to_string(), vec![4, 7]),
            Sector::new(7, "Command Base Beta".to_string(), vec![5, 6]),
        ]
    }
} 