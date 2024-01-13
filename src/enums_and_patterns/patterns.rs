#[derive(Default, Clone)]
struct Account {
    name: String,
    family: String,
    age: i32,
    typ: AccountType,
}

#[derive(Clone, Debug)]
enum AccountType {
    Admin(String, String),
    Normal(String),
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Normal(Default::default())
    }
}

fn refrence_pattern() {
    let acc = Account::default();
    let mut acc2 = Account::default();
    match acc {
        Account { ref name, .. } => {
            //ref
            let _name = name;
        }
    }
    match acc2 {
        Account { ref mut name, .. } => {
            //ref
            *name = "x".to_string();
        }
    }
    let _c = acc2.clone();
}

fn match_guard() {
    let acc = Some(Account::default());
    let _age = 21;
    match acc {
        None => {}
        Some(data) if data.age > _age => {}
        _ => {}
    }
}

fn at_sign_pattern() {
    fn process_admin(account_type: AccountType) {
        println!(" {account_type:?} is consumed here.");
    }
    fn process_age(age: i64) {
        println!("{age}");
    }

    let acc = Account::default();
    //form 1
    match acc.typ {
        AccountType::Admin(role, department) => process_admin(AccountType::Admin(role, department)),
        AccountType::Normal(_) => {}
    }
    let acc = Account::default();
    //alternative and ocompact
    match acc.typ {
        ref args @ AccountType::Admin(..) => process_admin(args.clone()),
        AccountType::Normal(_) => {}
    }

    // let _clone = acc.clone();
    //it works with range too
    match acc.age {
        grownup @ 12..=24 => process_age(grownup as i64),
        _ => {}
    }
    //patterns in other places ...as
    {
        //unpack struct
        let Account { age, name, .. } = acc.clone();
    }
}
pub fn binary_tree_challange() {
    #[derive(Debug)]
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }
    #[derive(Debug)]
    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    impl<T: PartialOrd> BinaryTree<T> {
        fn add(&mut self, value: T) {
            match self {
                BinaryTree::Empty => {
                    *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    }))
                }
                BinaryTree::NonEmpty(ref mut node) => {
                    if value <= node.element {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
                }
            }
        }
    }
    let mut tree = BinaryTree::Empty;
    tree.add(1);
    tree.add(2);
    tree.add(31);
    tree.add(5);
    tree.add(4);
    tree.add(44);
    tree.add(41);
    tree.add(42);

    tree.add(54);
    tree.add(97);

    println!("{tree:?}");
}
