/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
 package pkgswitch.nathalia.junco;

 import java.util.Scanner;
 
 /**
  *
  * @author Estudiantes
  */
 public class SwitchNathaliaJunco {
 
     /**
      * @param args the command line arguments
      */
     public static void main(String[] args) {
         Scanner scanner = new Scanner(System.in);
         
         System.out.println("ingrese el caso a escanear: ");
                 
         int dia = scanner.nextInt();
         
         switch (dia) {                
               case 1:
                   
                     System.out.println("verano");
                     break;
                     
                case 2:
                   
                     System.out.println("otoño");
                     break;
                     
                 case 3:
                   
                     System.out.println("invierno" );
                     break;    
                     
                 case 4:
                   
                     
                  default:
                      System.out.println("numero invalido" );
                     break;
         }    
     }    
                     
     
                 
               
                     
                 
                 
                     
     
 }