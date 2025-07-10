public class Entity {

    public string CharacterName { get; set; }

    public List<string> Inventory { get; set; }

    public bool IsAlive { get; set; }

    public object LastCheckpoint { get; set; }

    public int Level { get; set; }

    public Stats Stats { get; set; }

}
