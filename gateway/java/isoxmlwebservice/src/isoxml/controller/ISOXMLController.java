package isoxml.controller;

import java.io.IOException;
import java.util.HashMap;
import java.util.Properties;

import jakarta.servlet.ServletConfig;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

import org.apache.log4j.Logger;

import isoxml.utilities.RequestReader;
import isoxml.utilities.XMLParser;


/**
 * Entry-Point isoxmlwebservice
 *
 */
public class ISOXMLController extends HttpServlet {

    private static final long serialVersionUID = 2540095479247516488L;
    private String prefix;
    private Object tmpDir;
    private String parameterFile;
    private HashMap<String, ManagerInterface> managerHashMap;

    private static Logger isoxmlControllerLogger = Logger.getLogger(ISOXMLController.class.getName());

    @Override
    public void init(ServletConfig config) throws ServletException {
        super.init(config);

        prefix = getServletContext().getRealPath("/");
        tmpDir = getServletContext().getAttribute("jakarta.servlet.context.tempdir");
        parameterFile = prefix + config.getInitParameter("parameterFile");
        
        Properties applikationsParameter = new Properties();
        applikationsParameter.setProperty("parameterFile", parameterFile);
        
        managerHashMap = new HashMap<String, ManagerInterface>(1);
        ManagerInterface mi;
        try {
            mi = new TaskManager();
            mi.init(applikationsParameter, tmpDir);
            managerHashMap.put("task", mi);
        } catch (Exception e) {
            throw new ServletException("*** ISOXMLController [init] -> Manager can't be initialized: " + e.getMessage());
        }
    }

    public void processRequest(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
        try {
            HashMap<String, String> parameterMap = XMLParser.getKonfigurationMap(parameterFile, "Konfiguration/Security");
            String konfigurationhashKey = parameterMap.get("hashKey");
            // HashKey
            String requesthashKey = RequestReader.getStringParameter(request, "hashkey");
            if (requesthashKey != null && requesthashKey.equals(konfigurationhashKey)) {

                isoxmlControllerLogger.debug("Encoding before:" + request.getCharacterEncoding());
                request.setCharacterEncoding("UTF-8");
                isoxmlControllerLogger.debug("Encoding after:" + request.getCharacterEncoding());

                String managerNickname = RequestReader.getAction(request).toLowerCase();
                isoxmlControllerLogger.info("managerNickname: " + managerNickname);
                
                ManagerInterface manager = null;
                if (managerNickname != null && managerHashMap.containsKey(managerNickname)) {
                    manager = managerHashMap.get(managerNickname);
                }
                if (manager != null) {
                    try {
                        isoxmlControllerLogger.info("Forward: " + manager.getPackageClassName());
                        manager.processRequest(request, response, prefix);
                    } catch (Exception e) {
                        isoxmlControllerLogger.error("manager.processRequest: " + e.getMessage(), e);
                    }

                } else {
                    try {
                        request.getRequestDispatcher("/index.jsp").forward(request, response);
                    } catch (Exception se) {
                        throw new ServletException(se.getMessage());
                    }
                }
            }
        } catch (Exception e) {
            isoxmlControllerLogger.error("processRequest:"+e.getMessage());
        }
        
    }

    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
        processRequest(request, response);
    }

    @Override
    public void doPost(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
        processRequest(request, response);
    }
}
