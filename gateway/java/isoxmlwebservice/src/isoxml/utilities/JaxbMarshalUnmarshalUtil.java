package isoxml.utilities;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.InputStream;
import java.text.DecimalFormat;
import java.util.Iterator;

import javax.xml.XMLConstants;
import javax.xml.validation.Schema;
import javax.xml.validation.SchemaFactory;

import org.apache.log4j.Logger;
import org.xml.sax.SAXException;
import jakarta.xml.bind.MarshalException;

import isoxml.iso11783.CCG;
import isoxml.iso11783.CCT;
import isoxml.iso11783.CLD;
import isoxml.iso11783.CPC;
import isoxml.iso11783.CTP;
import isoxml.iso11783.CTR;
import isoxml.iso11783.DVC;
import isoxml.iso11783.FRM;
import isoxml.iso11783.ISO11783TaskData;
import isoxml.iso11783.OTQ;
import isoxml.iso11783.PDT;
import isoxml.iso11783.PFD;
import isoxml.iso11783.PGP;
import isoxml.iso11783.TSK;
import isoxml.iso11783.VPN;
import isoxml.iso11783.WKR;
import isoxml.iso11783.XFR;
import jakarta.xml.bind.JAXBContext;
import jakarta.xml.bind.JAXBException;
import jakarta.xml.bind.Marshaller;
import jakarta.xml.bind.Unmarshaller;

/**
 * helper class to marshal/unmarschal files 
 *
 */
public class JaxbMarshalUnmarshalUtil{
   private static Logger jaxBLogger = Logger.getLogger(JaxbMarshalUnmarshalUtil.class.getName());
    
   static final DecimalFormat DF_2 = new DecimalFormat( "#,##0.00" );
   
   public static <T> T unmarshalwebservice( String xsdSchema, InputStream is, Class<T> clss )
   throws JAXBException, SAXException
   {
      return unmarshal( xsdSchema, is, clss );
   }
   
   public static <T> T unmarshal( String xsdSchema, InputStream is, Class<T> clss )
   throws JAXBException, SAXException
   {
      // Schema und JAXBContext are  multi thread safe ("thread safe"):
      SchemaFactory schemaFactory = SchemaFactory.newInstance( XMLConstants.W3C_XML_SCHEMA_NS_URI );
      Schema        schema        = ( xsdSchema == null || xsdSchema.trim().length() == 0 )
                                    ? null : schemaFactory.newSchema( new File( xsdSchema ) );
      JAXBContext   jaxbContext   = JAXBContext.newInstance( clss.getPackage().getName() );
      return unmarshal( jaxbContext, schema, is, clss );
   }

   public static <T> T unmarshal( JAXBContext jaxbContext, Schema schema, InputStream is, Class<T> clss )
   throws JAXBException
   {
      // Unmarshaller is not multi thread safe:
      Unmarshaller unmarshaller = jaxbContext.createUnmarshaller();
      unmarshaller.setSchema( schema );
      return clss.cast( unmarshaller.unmarshal( is ) );
   }

   public static void marshal( String xsdSchema, String xmlDatei, Object jaxbElement )
   throws JAXBException, SAXException
   {
      SchemaFactory schemaFactory = SchemaFactory.newInstance( XMLConstants.W3C_XML_SCHEMA_NS_URI );
      Schema        schema        = ( xsdSchema == null || xsdSchema.trim().length() == 0 )
                                    ? null : schemaFactory.newSchema( new File( xsdSchema ) );
      JAXBContext   jaxbContext   = JAXBContext.newInstance( jaxbElement.getClass().getPackage().getName() );
      marshal( jaxbContext, schema, xmlDatei, jaxbElement );
   }

   public static void marshal( JAXBContext jaxbContext, Schema schema, String xmlDatei, Object jaxbElement )
   throws JAXBException
   {
      Marshaller marshaller;
      try {
        marshaller = jaxbContext.createMarshaller();
        marshaller.setSchema( schema );
        marshaller.setProperty( Marshaller.JAXB_ENCODING, "UTF-8" );
        marshaller.setProperty( Marshaller.JAXB_FORMATTED_OUTPUT, Boolean.TRUE );
        
        marshaller.marshal( jaxbElement, new File( xmlDatei ) );
      } catch (MarshalException e) {
          jaxBLogger.error("marshal:"+e.getMessage()); 
      } catch (Exception e) {
          jaxBLogger.error("marshal:"+e.getMessage());
      }
   }

   /** main()-Methode only for test */
   public static void main( String[] args ) throws JAXBException, SAXException, ClassNotFoundException
   {
      if( args.length != 3 ) {
         System.out.println( "\nPlease set XSD-Schema, XML-Document and Target-Class." );
         return;
      }
      System.out.println( "\nXSD-Schema: " + args[0] + ", XML-Document: " + args[1] + ", Target-Class: " + args[2] + "\n" );

      // Unmarshalling-Test:
      long startMemoryUsage = calculateMemoryUsage();
      long startTime = System.nanoTime();
      
      FileInputStream fis = null;
      try{
        fis = new FileInputStream(args[1]);
      }catch(FileNotFoundException fnfe){
          fnfe.printStackTrace();
      }
        
      Object obj = unmarshal( args[0], fis, Class.forName( args[2] ) );
      String duration = calculateDuration( startTime );
      String memoryUsage = formatMemorySize( calculateMemoryUsage() - startMemoryUsage );
      System.out.println( "ParsingMemoryUsage = " + memoryUsage + ", ParsingDuration = " + duration );
      System.out.println( obj.getClass() );
      // makes only sense if toString()-Method is useful:
      System.out.println( obj );

      // Marshalling-Test:
      startTime = System.nanoTime();
      marshal( args[0], args[1] + "-output.xml", obj );
      duration = calculateDuration( startTime );
      System.out.println( "\n'" + args[1] + "-output.xml' created in " + duration + "." );
   }

   static String calculateDuration( long startTimeNanoSec )
   {
      long durationMs = (System.nanoTime() - startTimeNanoSec) / 1000 / 1000;
      if( durationMs < 1000 ) return "" + durationMs + " ms";
      return DF_2.format( durationMs / 1000. ) + " s";
   }

   static long calculateMemoryUsage()
   {
      System.gc();
      System.gc();
      return Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory();
   }

   static String formatMemorySize( long bytes )
   {
      if( bytes < 0 ) return "0 Byte";
      if( bytes < 1024 ) return "" + bytes + " Byte";
      double b = bytes / 1024.;
      if( b < 1024. ) return DF_2.format( b ) + " KByte";
      return DF_2.format( b / 1024. ) + " MByte";
   }
   
   public static String createStringAsReport( ISO11783TaskData tasks ){
     String report = "** Report ISOXML Task **";
     report+="\n\nManagementSoftwareManufacture:"+tasks.getManagementSoftwareManufacturer();
     report+="\n\nDataTransferOrigin:"+tasks.getDataTransferOrigin();
     report+="\nManagementSoftwareVersion:"+tasks.getManagementSoftwareVersion();
     report+="\nTaskControllerManufacturer:"+tasks.getTaskControllerManufacturer();
     report+="\nTaskControllerVersion:"+tasks.getTaskControllerVersion();
     report+="\nVersionMajor:"+tasks.getVersionMajor();
     report+="\nVersionMinor:"+tasks.getVersionMinor();

      report+="\nxmlElements-Size:" + tasks.getXmlElements().size();
      Iterator<Object> it = tasks.getXmlElements().iterator();
      while(it.hasNext()){
        Object xmlElement = it.next();
        
        report+="\nxmlElemet_" + xmlElement.toString();
        
        if(xmlElement instanceof CTR){
          //report+="\n  CTR_: " + ((CTR)xmlElement). );
          
        }else if(xmlElement instanceof CCG){
          //report+="\n  CCG_: " + ((CCG)xmlElement). );
          
        }else if(xmlElement instanceof TSK){
          //report+="\n  TSK_: " + ((TSK)xmlElement). );
          
        }else if(xmlElement instanceof PDT){
          //report+="\n  PDT_: " + ((PDT)xmlElement). );
          
        }else if(xmlElement instanceof CPC){
          //report+="\n  CPC_: " + ((CPC)xmlElement). );
          
        }else if(xmlElement instanceof FRM){
          FRM fe = (FRM)xmlElement;
          report+="\n  FRM_A_B_C_D_E_F_G_H_: " + fe.getA()+":"+fe.getB()+":"+fe.getC()+":"+fe.getD()+":"+fe.getE()+":"+fe.getF()+":"+fe.getG()+":"+fe.getH();
          //report+="\n  FRM_: " + ((FRM)xmlElement). );
          
        }else if(xmlElement instanceof VPN){
          //report+="\n  VPN_: " + ((VPN)xmlElement). );
          
        }else if(xmlElement instanceof CLD){
          //report+="\n  CLD_: " + ((CLD)xmlElement). );
          
        }else if(xmlElement instanceof DVC){
          //report+="\n  DVC_: " + ((DVC)xmlElement). );
          
        }else if(xmlElement instanceof CTP){
          report+="\n  CTP_A: " + ((CTP)xmlElement).getA();
          report+="\n  CTP_B: " + ((CTP)xmlElement).getB();
          if(((CTP)xmlElement).getCVT() != null && ((CTP)xmlElement).getCVT().get(0) != null){
            report+="\n  CTP_CVT_A_B: " + ((CTP)xmlElement).getCVT().get(0).getA() + ((CTP)xmlElement).getCVT().get(0).getB();
          }
         //report+="\n  CTP_: " + ((CTP)xmlElement). );
        }else if(xmlElement instanceof PFD){
          //report+="\n  PFD_: " + ((PFD)xmlElement). );
          
        }else if(xmlElement instanceof PGP){
          //report+="\n  PGP_: " + ((PGP)xmlElement). );
          
        }else if(xmlElement instanceof OTQ){
          //report+="\n  OTQ_: " + ((OTQ)xmlElement). );
          
        }else if(xmlElement instanceof WKR){
          //report+="\n  WKR_: " + ((WKR)xmlElement). );
          
        }else if(xmlElement instanceof XFR){
          //report+="\n  XFR_: " + ((XFR)xmlElement). );
          
        }else if(xmlElement instanceof CCT){
          //report+="\n  CCT_: " + ((CCT)xmlElement). );
          
        }  
      }
      return report;
   }
}


