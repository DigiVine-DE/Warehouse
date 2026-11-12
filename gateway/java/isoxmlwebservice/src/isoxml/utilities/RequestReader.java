package isoxml.utilities;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;

import jakarta.servlet.http.HttpServletRequest;

import org.apache.log4j.Logger;

/**
 * helper class to simulate rest service library
 * TODO: replaced by rest service library, after decision is made
 *
 */
public class RequestReader 
{
	@SuppressWarnings ("unused")
    private static Logger utilitiesLogger = Logger.getLogger(RequestReader.class.getName());
	
	public static String getAction(HttpServletRequest request) {
        String action = null;
        if (request.getPathInfo() == null || request.getPathInfo().equals("") || request.getPathInfo().length() < 1) {
            action = "";
        } else if (request.getPathInfo().indexOf('/', 1) == -1) {
            action = request.getPathInfo().substring(1);
        } else {
            action = request.getPathInfo().substring(1, request.getPathInfo().indexOf('/', 1));
        }
        return action.toLowerCase();
    }
	
	public static String getSubAction(HttpServletRequest request) {
        String subaction;
        if (request.getPathInfo() == null || request.getPathInfo().indexOf('/', 1) == -1) {
            subaction = "";
        } else {
            subaction = request.getPathInfo().substring(request.getPathInfo().indexOf('/', 1) + 1);
            if (subaction.indexOf('/') != -1)
                subaction = subaction.substring(0, subaction.indexOf('/', 0));
        }
        return subaction.toLowerCase();
	}
	
	public static String getSubSubAction(HttpServletRequest request) {
        String subsubaction;
        if (request.getPathInfo() == null || request.getPathInfo().indexOf('/', 1) == -1) {
            subsubaction = "";
        } else {
            subsubaction = request.getPathInfo().substring(request.getPathInfo().indexOf('/', 1) + 1);
            if (subsubaction.indexOf('/', 1) == -1) {
                return subsubaction = "go";
            } else {
                subsubaction = subsubaction.substring(subsubaction.indexOf('/', 0) + 1);
                if (subsubaction.indexOf('/') != -1)
                    subsubaction = subsubaction.substring(0, subsubaction.indexOf('/', 0));
            }
        }
        return subsubaction.toLowerCase();
    }
    /**
     * To convert the InputStream to String we use the BufferedReader.readLine()
     * method. We iterate until the BufferedReader return null which means
     * there's no more data to read. Each line will appended to a StringBuilder
     * and returned as String.
     */
	public static String convertStreamToString(InputStream is) throws IOException {
   
        if (is != null) {
              StringBuilder sb = new StringBuilder();
              String line;
    
              try {
                  BufferedReader reader = new BufferedReader(new InputStreamReader(is, "UTF-8"));
                while ((line = reader.readLine()) != null) {
                    sb.append(line).append("\n");
                }
            } finally {
                is.close();
            }
            return sb.toString();
        } else {       
            return "";
        }
	}
	
	public static String getStringParameter(HttpServletRequest request, String name) {
    Object value = request.getParameterMap().get(name);
    if (value != null) {
         if (value.getClass().isArray()){
             if((((String[])value)[0]).length() > 0)
               return ((String[]) value)[0].trim();
             else
               return null;
         }else{
             if(((String)value).length() > 0)
               return (String) value;
             else 
               return null;
         }
    
    } 
    return null;
 }
}
