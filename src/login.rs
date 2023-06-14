use chrono;

pub fn login(sender_id:String,username:String, password:String,receiving_firm:String,receiving_target:String) -> String{
    let login:String = format!("8=FIX.4.4|9=114| 35=A |34=1| 49={sender_id}| 52=20120927-13:15:34.754| 56={receiving_firm} |57={receiving_target}| 553={username}| 554={password}| 98=0 |108=30 |141=Y| 10=146|");
    login
}